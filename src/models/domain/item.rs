use std::fmt::Display;

use chrono::Local;
use prettytable::{row, Row};

use crate::{models::uuid::Uuid, types::ContractsList};

use super::{contract::Contract, ToRow};

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Tool,
    Vehicle,
    Game,
    Toy,
    Sport,
    Other,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("{}", self.to_string().as_str()))
        match *self {
            Category::Tool => f.write_str("Tool"),
            Category::Vehicle => f.write_str("Vehicle"),
            Category::Game => f.write_str("Game"),
            Category::Toy => f.write_str("Toy"),
            Category::Sport => f.write_str("Sport"),
            Category::Other => f.write_str("Other"),
        }
    }
}

impl Default for Category {
    fn default() -> Self {
        Category::Other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub uuid: Uuid,
    pub category: Category,
    pub name: String,
    pub description: String,
    pub active_contract: Option<Contract>,
    pub history: ContractsList,
    day_of_creation: chrono::DateTime<Local>,
    cost_per_day: f64,
}

impl Item {
    pub fn new(name: String, description: String, category: Category, cost_per_day: f64) -> Item {
        Item {
            name,
            category,
            description,
            active_contract: None,
            cost_per_day,
            uuid: Uuid::new(),
            history: vec![],
            day_of_creation: chrono::offset::Local::now(),
        }
    }

    pub fn name(mut self, name: String) -> Item {
        self.name = name;
        self
    }

    pub fn category(mut self, category: Category) -> Item {
        self.category = category;
        self
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("Name:\t{}\nCategory:\t{}\nDescription:\t{}Contract:\t{:?}\nDay of creation:\t{}\nCost per Day:\t{}", self.name, self.category, self.description, self.contract, self.day_of_creation, self.cost_per_day))
        f.write_fmt(format_args!(
            "Item [\n\t  Name:\t\t{}\n\t  Description:\t{}\n\t  Category:\t{},\n\t  Contract:\t{:?}\n\t  Date:\t\t{}\n\t  Cost per Day:\t${}\n\t]",
            self.name,
            self.description,
            self.category,
            self.active_contract,
            self.day_of_creation.date().naive_local(),
            self.cost_per_day,
        ))
    }
}

impl ToRow for Item {
    fn to_row(&self) -> Row {
        let contract_str = match &self.active_contract {
            Some(c) => c.uuid.to_string(),
            None => "No Contract".to_owned(),
        };
        row![
            self.name.clone(),
            self.description.clone(),
            self.category.clone().to_string(),
            contract_str,
            self.cost_per_day.to_string(),
        ]
    }
}