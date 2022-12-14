use super::{
    contract::Contract,
    item::{Category, Item},
    member::Member,
};
use crate::{
    errors::SysError,
    models::uuid::Uuid,
    types::{Demo, SysResult},
};
use shared::{Builder, Model};
use std::collections::HashMap;

/// All methods for the lending system.
pub trait LendingSystem {
    /// Gets all the members in the system.
    fn get_members(&self) -> Vec<&Member>;
    /// Gets a specific member.
    fn get_member(&self, member: &Member) -> SysResult<Member>;
    /// Returns a mutable version of a member.
    fn get_member_mut(&mut self, member: &Member) -> SysResult<&mut Member>;
    /// Adds a member to the system.
    fn add_member(&mut self, member: Member) -> SysResult<()>;
    /// Removes a member from the system.
    fn remove_member(&mut self, member: &Member) -> SysResult<()>;
    /// updates a member with the new information.
    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> SysResult<()>;
    /// checks if the member passed into function acutally exists in this system.
    fn exists_member(&self, member: &Member) -> bool;
    /// Gets all the items in the system.
    fn get_items(&self) -> Vec<&Item>;
    /// Gets all the items for a specific member.
    fn get_items_for_member(&self, member: &Member) -> Vec<&Item>;
    /// Returns Some if item exists otherwise returns None.
    fn get_item_for_contract(&self, contract: &Contract) -> Option<&Item>;
    /// Returns Some if item exists otherwise returns None.
    fn get_item(&self, item: &Item) -> SysResult<Item>;
    /// Gets contract for.
    fn get_contract(&self, contract: &Contract) -> SysResult<Contract>;
    /// Adds item to the system.
    fn add_item(&mut self, item: Item) -> SysResult<()>;
    /// Removes item from the system.
    fn remove_item(&mut self, item: &Item) -> SysResult<()>;
    /// Updates item with the new information.
    fn update_item(&mut self, info: &Item) -> SysResult<()>;
    /// Counts the number of items for a certain member.
    fn count_items_for_member(&self, member: &Member) -> usize;
    /// Increments system day counter and calls all required methods to update contracts
    /// items and members information.
    fn incr_time(&mut self) -> SysResult<()>;
    /// Gets current time.
    fn now(&self) -> usize;
}

/// system struct.
#[derive(Debug, Clone, Model, Builder)]
pub struct System {
    members: HashMap<Uuid, Member>,
    items: HashMap<Uuid, Item>,
    day: usize,
}

impl System {
    /// Creates a new system instance.
    pub fn new() -> System {
        System {
            members: HashMap::new(),
            items: HashMap::new(),
            day: 0,
        }
    }
}

impl LendingSystem for System {
    fn get_members(&self) -> Vec<&Member> {
        self.members
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<&Member>>()
    }

    fn get_member(&self, member: &Member) -> SysResult<Member> {
        match self.members.get(member.get_uuid()) {
            Some(m) => Ok(m.clone()),
            None => Err(SysError::DoesntExist),
        }
    }

    fn get_member_mut(&mut self, member: &Member) -> SysResult<&mut Member> {
        match self.members.get_mut(member.get_uuid()) {
            Some(m) => Ok(m),
            None => Err(SysError::DoesntExist),
        }
    }

    fn add_member(&mut self, member: Member) -> SysResult<()> {
        if self.exists_member(&member) {
            return Err(SysError::AlreadyExists);
        }
        self.members.insert(member.get_uuid().clone(), member);
        Ok(())
    }

    fn remove_member(&mut self, member: &Member) -> SysResult<()> {
        if !self.exists_member(member) {
            return Err(SysError::DoesntExist);
        }
        self.members.remove(member.get_uuid());
        Ok(())
    }

    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> SysResult<()> {
        if !self.exists_member(old_info) {
            return Err(SysError::DoesntExist);
        }
        *self.members.get_mut(old_info.get_uuid()).unwrap() = new_info.clone();
        Ok(())
    }

    fn exists_member(&self, member: &Member) -> bool {
        self.members.iter().any(|entry| {
            let m = entry.1.clone();
            m == member.clone()
        })
    }

    fn get_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<&Item>>()
    }

    fn get_items_for_member(&self, member: &Member) -> Vec<&Item> {
        self.get_items()
            .into_iter()
            .filter(|item| item.get_owner() == member)
            .collect::<Vec<&Item>>()
    }

    fn get_item_for_contract(&self, contract: &Contract) -> Option<&Item> {
        for (_, item) in self.items.iter() {
            for c in item.get_history().iter() {
                if c.get_uuid() == contract.get_uuid() {
                    return Some(item);
                }
            }
        }
        None
    }

    fn get_item(&self, item: &Item) -> SysResult<Item> {
        if !self.items.contains_key(item.get_uuid()) {
            return Err(SysError::DoesntExist);
        }
        Ok(self.items[item.get_uuid()].clone())
    }

    fn get_contract(&self, contract: &Contract) -> SysResult<Contract> {
        for item in self.items.values() {
            if item
                .get_history()
                .iter()
                .any(|c| c.get_uuid() == contract.get_uuid())
            {
                return match item.get_history().get(contract) {
                    Some(c) => Ok(c.clone()),
                    None => Err(SysError::DoesntExist),
                };
            }
        }
        Err(SysError::DoesntExist)
    }

    fn add_item(&mut self, item: Item) -> SysResult<()> {
        match self.items.insert(item.get_uuid().clone(), item.clone()) {
            Some(_) => Err(SysError::AlreadyExists),
            None => {
                let temp = self.get_member(item.get_owner());
                match temp {
                    Ok(mut member) => match member.add_credits(100f64) {
                        Ok(_) => match self.update_member(item.get_owner(), &member) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err),
                        },
                        Err(_) => Err(SysError::CannotUpdate),
                    },
                    Err(err) => Err(err),
                }
            }
        }
    }

    fn remove_item(&mut self, item: &Item) -> SysResult<()> {
        match self.items.remove(&item.get_uuid().clone()) {
            Some(_) => Ok(()),
            None => Err(SysError::CannotDelete),
        }
    }

    fn update_item(&mut self, info: &Item) -> SysResult<()> {
        match self.items.get_mut(info.get_uuid()) {
            Some(_) => {
                *self.items.get_mut(info.get_uuid()).unwrap() = info.clone();
                Ok(())
            }
            None => Err(SysError::CannotUpdate),
        }
    }

    fn count_items_for_member(&self, member: &Member) -> usize {
        self.get_items().iter().fold(0, |cnt, item| {
            if item.get_owner() == member {
                cnt + 1
            } else {
                cnt
            }
        })
    }

    fn incr_time(&mut self) -> SysResult<()> {
        self.day += 1;
        let cp = self.clone();
        let mut out: SysResult<()> = Ok(());
        for item in cp.get_items() {
            let res = match item.get_active_contract(self.day) {
                Some(con) => {
                    let owner = self.get_member(con.get_owner());
                    let res1 = match owner {
                        Ok(mut o) => match o.add_credits(*item.get_cost_per_day()) {
                            Ok(_) => match self.update_member(con.get_owner(), &o) {
                                Err(err) => Err(err),
                                Ok(_) => Ok(()),
                            },
                            Err(_) => Err(SysError::CannotUpdate),
                        },
                        Err(_) => Ok(()),
                    };
                    let lendee = self.get_member(con.get_lendee());
                    let res2 = match lendee {
                        Ok(mut l) => match l.deduce_credits(*item.get_cost_per_day()) {
                            Ok(_) => match self.update_member(con.get_lendee(), &l) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err),
                            },
                            Err(_) => Err(SysError::CannotUpdate),
                        },
                        Err(_) => Err(SysError::CannotUpdate),
                    };
                    match (res1, res2) {
                        (Ok(_), Ok(_)) => Ok(()),
                        _ => Err(SysError::CannotUpdate),
                    }
                }
                None => Ok(()),
            };
            out = res
        }
        out
    }

    fn now(&self) -> usize {
        self.day
    }
}

impl Demo for System {
    fn init_demo(&mut self) {
        let sys = Self::new();

        let members = vec![
            Member::new(
                "Allan".to_owned(),
                "allan@enigma.com".to_owned(),
                "0123456789".to_owned(),
                sys.now(),
            )
            .expect("Should not fail"),
            Member::new(
                "Tina".to_owned(),
                "tina@somethingelse.com".to_owned(),
                "01234543210".to_owned(),
                sys.now(),
            )
            .expect("Should not fail."),
            Member::new(
                "Turing".to_owned(),
                "turing@enigma.com".to_owned(),
                "9876567890".to_owned(),
                sys.now(),
            )
            .expect("Should not fail."),
            Member::new(
                "Jeff".to_owned(),
                "jeff@bezos.com".to_owned(),
                "0987654321".to_owned(),
                sys.now(),
            )
            .expect("Should not fail."),
        ];

        let mut items = vec![
            Item::new(
                "Monopoly".to_owned(),
                "Family Game".to_owned(),
                Category::Game,
                members[0].clone(),
                30f64,
                sys.now(),
            ),
            Item::new(
                "Siedler".to_owned(),
                "Another Family Game".to_owned(),
                Category::Game,
                members[0].clone(),
                45f64,
                sys.now(),
            ),
            Item::new(
                "T-Rex".to_owned(),
                "Dinosaur".to_owned(),
                Category::Toy,
                members[2].clone(),
                10f64,
                sys.now(),
            ),
            Item::new(
                "Hammer".to_owned(),
                "A useful tool".to_owned(),
                Category::Tool,
                members[1].clone(),
                150f64,
                sys.now(),
            ),
        ];

        let contracts = vec![
            Contract::new(
                items[2].get_owner().clone(),
                members[1].clone(),
                sys.now() + 6,
                6,
                items[2].get_cost_per_day() * 6f64,
            ),
            Contract::new(
                items[1].get_owner().clone(),
                members[1].clone(),
                sys.now() + 12,
                9,
                items[1].get_cost_per_day() * 9f64,
            ),
            Contract::new(
                items[3].get_owner().clone(),
                members[2].clone(),
                sys.now(),
                10,
                items[3].get_cost_per_day() * 10f64,
            ),
            Contract::new(
                items[1].get_owner().clone(),
                members[1].clone(),
                sys.now(),
                5,
                items[1].get_cost_per_day() * 20f64,
            ),
        ];

        for member in members.iter() {
            self.add_member(member.clone()).expect("");
            let mut temp = member.clone();
            temp.add_credits(700f64).expect("");
            self.update_member(member, &temp).expect("");
        }
        items[0].add_contract(contracts[0].clone()).expect("");
        items[1].add_contract(contracts[1].clone()).expect("");
        items[1].add_contract(contracts[2].clone()).expect("");
        items[2].add_contract(contracts[3].clone()).expect("");
        for item in items.iter() {
            self.add_item(item.clone()).expect("");
        }
    }
}
