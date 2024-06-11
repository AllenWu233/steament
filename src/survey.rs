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

impl Survey {
    pub fn new(sections: Vec<Section>) -> Self {
        Survey { sections }
    }

    pub fn print_result(&self) -> String {
        let mut result = String::new();
        for section in &self.sections {
            result += &section.get_result(true);
            result += "\n";
        }
        print!("{}", &result);
        result
    }

    pub fn run(&mut self) {
        let mut i = 0; // Section counter
        while i < self.sections.len() {
            let item = &mut self.sections[i];
            print!("{}", item.get_result(false));
            print!("(输入若干个序号或区间，用空格分隔。如：1 3-4 6)\n(输入b返回上一步)请选择：");

            match read_user_input() {
                Some(change_list) if change_list.is_empty() => i = i.saturating_sub(1),
                Some(change_list) => {
                    item.change_checkboxes(change_list);
                    i += 1;
                }
                _ => {
                    println!("不合法输入，请重试。");
                }
            }
        }
        export_to_clipboard(self.print_result());
    }
}

fn read_user_input() -> Option<Vec<usize>> {
    todo!()
}

fn export_to_clipboard(result: String) {
    todo!()
}
