use std::io::{self, Write};

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
    fn get_result(&self, checkbox: bool) -> String {
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
    fn change_checkboxes(&mut self, change_list: Vec<usize>) {
        for i in change_list {
            self.items[i].checked = true;
        }
    }
}

impl Survey {
    pub fn new(sections: Vec<Section>) -> Self {
        Survey { sections }
    }

    // Print the whole comment and return a string
    fn print_result(&self) -> String {
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
            print!("\n{}", item.get_result(false));
            println!(
                "# 输入若干个序号或区间，用空格分隔。如：1 3-4 6\n# 输入b返回上一步。请选择："
            );
            let _ = io::stdout().flush();
            match get_change_list(item.items.len()) {
                // Previous
                Some(change_list) if change_list.is_empty() => i = i.saturating_sub(1),
                // Next
                Some(change_list) => {
                    item.change_checkboxes(change_list);
                    i += 1;
                }
                _ => println!("不合法输入，请重试。"),
            }
        }
        self.print_result();
        // export_to_clipboard(self.print_result());
    }
}

// Read from stdin and return a string
fn input() -> String {
    // Read from stdin
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);
    String::from(buffer.trim()) // Remove '\n' in the end
}

// Get a change list from user input
fn get_change_list(len: usize) -> Option<Vec<usize>> {
    // Previous section
    let buffer = input();
    if buffer == "b" {
        Some(Vec::new())
    }
    // Next section
    else {
        let tmp: Vec<_> = buffer.split(' ').collect();
        let mut change_list: Vec<usize> = Vec::new();
        for i in tmp {
            // Single option
            if let Ok(index) = String::from(i).parse::<usize>() {
                if index >= len {
                    return None;
                }
                change_list.push(index);
            }
            // Adjective
            else if let Some(t) = i.find('-') {
                if let (Ok(left), Ok(right)) = (
                    String::from(i)[..t].parse::<usize>(),
                    String::from(i)[t + 1..].parse::<usize>(),
                ) {
                    if left <= right && right < len {
                        for index in left..=right {
                            change_list.push(index);
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        // Unique the vector
        let unique = std::collections::BTreeSet::from_iter(change_list);
        let mut change_list = unique.into_iter().collect::<Vec<_>>();
        change_list.sort_unstable();

        Some(change_list)
    }
}

fn export_to_clipboard(result: String) {
    todo!()
}
