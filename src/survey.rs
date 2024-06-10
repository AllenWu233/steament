const EMPTY_CHECKBOX: &str = "☐";
const SELECT_CHECKBOX: &str = "✓";

// Single option
pub struct Item {
    content: String,
    checked: bool,
}

pub struct Section {
    title: String,
    items: Vec<Item>,
}

pub struct Survey {
    sections: Vec<Section>,
}

impl Item {
    pub fn new(content: String) -> Self {
        Item {
            content,
            checked: false,
        }
    }
}

impl Section {
    pub fn new(title: String, items: Vec<Item>) -> Self {
        Section { title, items }
    }

    // Print the whole section with index or checkboxes
    pub fn print_section(&self, checkbox: bool) {
        println!("\n======={}=======", self.title);
        let mut i = 0;
        for item in &self.items {
            if checkbox {
                println!(
                    "{} {}",
                    {
                        if item.checked {
                            SELECT_CHECKBOX
                        } else {
                            EMPTY_CHECKBOX
                        }
                    },
                    item.content
                )
            } else {
                println!("{}. {}", i, item.content);
                i += 1;
            }
        }
    }

    // Select the checkboxes
    pub fn change_checkboxes(&mut self, change_list: Vec<usize>) {
        for i in change_list {
            self.items[i].checked = true;
        }
    }
}

impl Survey {
    pub fn new(sections: Vec<Section>) -> Self {
        Survey { sections }
    }

    fn read_user_input(&self) -> Option<Vec<usize>> {
        todo!()
    }

    fn print_result(&self) {
        todo!()
    }

    fn export_to_clipboard(&self) {
        todo!()
    }

    pub fn run(&mut self) {
        let mut i = 0; // Part counter
        while i < self.sections.len() {
            println!("(输入若干个序号或区间，用空格分隔。如：1 3-4 6)\n(输入b返回上一步)请选择：");
            if let Some(change_list) = self.read_user_input() {
                let item = &mut self.sections[i];
                item.change_checkboxes(change_list);
                i += 1;
            } else {
                i = i.saturating_sub(1);
            }
        }
        self.print_result();
        self.export_to_clipboard();
    }
}
