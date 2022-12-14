#[cfg(test)]
mod system_tests {
    use crate::{
        errors::SysError,
        models::domain::{
            contract::Contract,
            item::{Category, Item},
            member::Member,
            system::{LendingSystem, System},
        },
    };

    #[test]
    fn test_add_member() {
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "0123456789".to_owned(),
            0,
        )
        .expect("Should not fail");
        let mut system = System::new();
        let r1 = system.add_member(allan);
        assert_eq!(r1, Ok(()));
    }

    #[test]
    fn test_add_member_duplicate_email() {
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "0123456789".to_owned(),
            0,
        )
        .expect("Should not fail");

        let turing1 = Member::new(
            "Turing".to_owned(),
            "allan@enigma.com".to_owned(),
            "012345678901".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let mut system = System::new();
        let r0 = system.add_member(allan);
        assert_eq!(r0, Ok(()));
        let r1 = system.add_member(turing1);
        assert_eq!(r1, Err(SysError::AlreadyExists));
    }

    #[test]
    fn test_add_member_duplicate_phone_nr() {
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "0123456789".to_owned(),
            0,
        )
        .expect("Should not fail");

        let turing2 = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "0123456789".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let mut system = System::new();
        let r0 = system.add_member(allan);
        assert_eq!(r0, Ok(()));
        let r1 = system.add_member(turing2);
        assert_eq!(r1, Err(SysError::AlreadyExists));
    }

    #[test]
    fn test_multiple_validd_members() {
        let mut system = System::new();
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "0123456789".to_owned(),
            system.now(),
        )
        .expect("Should not fail");
        let turing1 = Member::new(
            "Turing".to_owned(),
            "allan@somethingelse.com".to_owned(),
            "01234543210".to_owned(),
            system.now(),
        )
        .expect("Should not fail.");
        let turing2 = Member::new(
            "Turing".to_owned(),
            "turing2@enigma.com".to_owned(),
            "9876567890".to_owned(),
            system.now(),
        )
        .expect("Should not fail.");
        let turing3 = Member::new(
            "Turing".to_owned(),
            "another@turing.com".to_owned(),
            "0987654321".to_owned(),
            system.now(),
        )
        .expect("Should not fail.");

        let r1 = system.add_member(allan);
        assert_eq!(r1, Ok(()));

        let r2 = system.add_member(turing1);
        assert_eq!(r2, Ok(()));

        let r3 = system.add_member(turing2);
        assert_eq!(r3, Ok(()));

        let r4 = system.add_member(turing3);
        assert_eq!(r4, Ok(()));
    }

    #[test]
    fn test_exists_member() {
        let turing: Member = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "0123456789".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "1235678999".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.exists_member(&turing);
        assert_eq!(r1, true);

        let r2 = system.exists_member(&allan);
        assert_eq!(r2, false);
    }

    #[test]
    fn test_remove_member() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "12345678909".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.exists_member(&turing);
        assert_eq!(r1, true);

        match system.remove_member(&turing) {
            Ok(_) => {}
            Err(_) => assert!(false),
        }

        let r2 = system.exists_member(&turing);
        assert_eq!(r2, false);
    }

    #[test]
    fn test_create_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "01234567890".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let item = Item::new(
            "Monopoly".to_owned(),
            "A beautiful Family Game.".to_owned(),
            Category::Game,
            turing.clone(),
            20f64,
            0,
        );

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item);
        assert_eq!(r1, Ok(()))
    }

    #[test]
    fn test_exists_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "1234567890".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let item = Item::new(
            "Monopoly".to_owned(),
            "A beautiful Family Game.".to_owned(),
            Category::Game,
            turing.clone(),
            20f64,
            0,
        );

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        let r2 = system.get_item(&item);
        println!("{:#?}", system.get_items());
        assert_eq!(r2.is_ok(), true);
    }

    #[test]
    fn test_delete_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "1234567890".to_owned(),
            0,
        )
        .expect("Should not fail.");

        let item = Item::new(
            "Monopoly".to_owned(),
            "A beautiful Family Game.".to_owned(),
            Category::Game,
            turing.clone(),
            20f64,
            0,
        );

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        let r2 = system.remove_item(&item);
        assert_eq!(r2, Ok(()));
    }

    #[test]
    fn test_advance_time() {
        let mut sys = System::new();
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4602134567".to_owned())
            .build();
        let bob = Member::default()
            .name("Bob".to_owned())
            .email("bob@gmail.com".to_owned())
            .phone_nr("46291328475".to_owned())
            .credits(300f64)
            .build();
        let mut monopoly = Item::default()
            .name("Monopoly".to_owned())
            .description("A Family Game".to_owned())
            .category(Category::Game)
            .cost_per_day(20f64)
            .owner(allan.clone());

        let c1 = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(5f64 * monopoly.get_cost_per_day())
            .from_date(0, 6)
            .build();

        monopoly.add_contract(c1).expect("");
        sys.add_member(allan.clone()).expect("");
        sys.add_member(bob.clone()).expect("");
        sys.add_item(monopoly).expect("");

        // Since allan is the owner, he should have 100 credits.
        assert_eq!(sys.get_member(&allan).unwrap().get_credits(), &100f64);
        // Since bob is not an owner of any items, he should only have the 300
        // credits we set when building.
        assert_eq!(sys.get_member(&bob).unwrap().get_credits(), &300f64);
        assert_eq!(sys.now(), 0);

        for _ in 0..8 {
            sys.incr_time().expect("");
        }

        println!("Allan: {}\nBob:{}", allan.get_credits(), bob.get_credits());

        assert_eq!(sys.now(), 8);
        assert_eq!(sys.get_member(&allan).unwrap().get_credits(), &200f64);
        assert_eq!(sys.get_member(&bob).unwrap().get_credits(), &200f64);
    }
}
