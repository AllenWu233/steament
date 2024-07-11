use super::item::Item;
use crate::utils::value::{EMPTY_CHECKBOX, SELECT_CHECKBOX};

pub struct Section {
    title: String,
    pub items: Vec<Item>,
}

impl Section {
    pub fn new(title: String, items: Vec<Item>) -> Self {
        Section { title, items }
    }

    // Return a string of the section with index or checkbox
    pub fn get_result(&self, checkbox: bool) -> String {
        let mut result = format!("======={}=======\n", self.title);
        let mut i = 0;
        for item in &self.items {
            if checkbox {
                result += &format!(
                    "{} {}\n",
                    {
                        if item.checked {
                            SELECT_CHECKBOX
                        } else {
                            EMPTY_CHECKBOX
                        }
                    },
                    item.content
                );
            } else {
                result += &format!("{}. {}\n", i, item.content);
                i += 1;
            }
        }
        result
    }

    // Select the checkboxes
    pub fn change_checkboxes(&mut self, change_list: Vec<usize>) {
        for i in change_list {
            self.items[i].checked = true;
        }
    }
}
