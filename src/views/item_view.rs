use super::console::{Console, Ui};
use super::Options;
use crate::models::domain::item::Category;
use crate::types::View;
use crate::{
    models::domain::{item::Item, Data, FromMap},
    types::StringMap,
};
use prettytable::Table;
use shared::COptions;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum ItemMenuOption {
    DisplayItemInfo,
    EditItemInfo,
    GetItemInfo,
    #[other]
    Other,
}

pub trait ItemView {
    fn item_menu(&self) -> ItemMenuOption;
    fn display_item_info(&self, item: Item);
    fn edit_item_info(&self, item: Item) -> Item;
    fn get_item_info(&self) -> Item;
}

pub struct CliItemView {
    console: Console,
}

impl CliItemView {
    pub fn new() -> CliItemView {
        CliItemView {
            console: Console::new(),
        }
    }
}

impl View for CliItemView {}

impl ItemView for CliItemView {
    fn item_menu(&self) -> ItemMenuOption {
        self.console.title();
        let choice: ItemMenuOption = self.console.show_menu(ItemMenuOption::options());
        match choice {
            ItemMenuOption::Other => self.item_menu(),
            _ => choice,
        }
    }

    fn display_item_info(&self, item: Item) {
        let mut table = Table::new();
        table.add_row(item.to_row());
        self.console.display_table(table);
    }

    fn edit_item_info(&self, item: Item) -> Item {
        let new_item_info = self
            .console
            .get_consecutive_str_input(Item::head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_item_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        item.copy_with(data)
    }

    fn get_item_info(&self) -> Item {
        let [name, description, category, cost_per_day] = <[String; 4]>::try_from(
            self.console
                .get_consecutive_str_input(Item::head_allowed_mutable())
                .iter()
                .map(|entry| -> String { entry.1.clone() })
                .collect::<Vec<String>>(),
        )
        .ok()
        .unwrap();
        Item::default()
            .name(name)
            .description(description)
            .category(Category::from_str(category.as_str()).unwrap())
            .cost_per_day(cost_per_day.parse::<f64>().unwrap())
            .build()
    }
}
