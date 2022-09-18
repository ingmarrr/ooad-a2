use std::{
    io::{self, stdin, Write},
    mem,
};

use crate::models::{member::Member, uuid::Uuid};

pub trait MemberDisplay {
    fn display_member_verbose(&self, member: Member);
    fn display_member_simple(&self, member: Member);
    fn ls_simple(&self, members: Vec<Member>);
    fn ls_verbose(&self, members: Vec<Member>);
    fn get_member_info(&self) -> Member;
    fn edit_member_info(&self, member: &mut Member) -> Member;
    fn get_str_input(&self, display: &str) -> String;
}

pub struct MemberView;

impl MemberView {
    pub fn new() -> MemberView {
        MemberView {}
    }
}

impl MemberDisplay for MemberView {
    fn display_member_verbose(&self, member: Member) {
        let mut items_str = String::new();
        if member.items.len() == 0 {
            items_str.push_str(" []")
        }
        for item in member.items.iter() {
            // let formatted = format!("\n\t{}", item);
            let formatted = format!("\n\t{},", item);
            items_str.push_str(&formatted);
        }
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nPhone number:\t{}\nCredits:\t{}\nItems [{}\n]",
            member.name, member.email, member.phone_nr, member.credits, items_str
        );
        println!("{out}");
    }

    fn display_member_simple(&self, member: Member) {
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nCredits:\t{}\nItems:\t\t{}\n",
            member.name,
            member.email,
            member.credits,
            member.items.len(),
        );
        println!("{out}");
    }

    fn ls_simple(&self, members: Vec<Member>) {
        for m in members {
            self.display_member_simple(m);
        }
    }

    fn ls_verbose(&self, members: Vec<Member>) {
        for m in members {
            self.display_member_verbose(m);
        }
    }

    fn get_member_info(&self) -> Member {
        Member::default()
            .name(self.get_str_input("Name: "))
            .email(self.get_str_input("Email: "))
            .phone_nr(self.get_str_input("Phone number: "))
            .clone()
    }

    fn edit_member_info(&self, member: &mut Member) -> Member {
        todo!()
    }

    fn get_str_input(&self, display: &str) -> String {
        print!("{display}");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => println!("There was some error displaying to console: {err}"),
        }
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(_) => println!("There was a problem reading the input"),
        };
        buf
    }
}
