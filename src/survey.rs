use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

const EMPTY_CHECKBOX: &str = "☐";
const SELECT_CHECKBOX: &str = "✓";
const TAIL: &str = "——该评论使用[url=https://github.com/AllenWu233/steament]steament[/url]生成";

#[derive(Clone)]
// Single option
struct Item {
    content: String,
    checked: bool,
}
struct Section {
    title: String,
    items: Vec<Item>,
}

pub struct Survey {
    game: String,
    sections: Vec<Section>,
}

impl Item {
    fn new(content: String) -> Self {
        Item {
            content,
            checked: false,
        }
    }
}

impl Section {
    fn new(title: String, items: Vec<Item>) -> Self {
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
    fn new(game: String, sections: Vec<Section>) -> Self {
        Survey { game, sections }
    }

    // Get the whole comment
    fn get_result(&self) -> String {
        let mut result = format!("游戏名：{}\n", self.game);
        for section in &self.sections {
            result += &section.get_result(true);
            result += "\n";
        }
        result + TAIL
    }

    // Read comment template from specific file to create a Survey
    pub fn init_from_file(filename: &str) -> Self {
        let file_content = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("File {} not existed!", filename));
        let messages: Vec<_> = file_content.lines().collect();
        let mut title = "";
        let mut sections: Vec<Section> = Vec::new();
        let mut items: Vec<Item> = Vec::new();
        for message in messages {
            // if message.len() >= 7 && &message[..7] == "Title: " {
            if utf8_slice::len(message) >= 7 && utf8_slice::till(message, 7) == "Title: " {
                title = utf8_slice::from(message, 7);
                // items = Vec::new();
                items.clear();
            } else if message == "######" {
                sections.push(Section::new(String::from(title), items.clone()));
            } else if message.is_empty() {
            } else {
                items.push(Item::new(String::from(message)));
            }
        }
        Survey::new(String::new(), sections)
    }

    pub fn run(&mut self) {
        self.game = loop {
            print!("# 游戏名：");
            let _ = io::stdout().flush();
            if let Ok(game) = input() {
                break game;
            } else {
                println!("# 不合法输入，请重试。");
            }
        };

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
                _ => println!("# 不合法输入，请重试。"),
            }
        }

        let result = self.get_result();
        println!("\n{}", &result);
        if export_to_clipboard(result).is_ok() {
            println!("\n# 结果已复制到剪贴板！");
        } else {
            println!("\n# 结果未能复制到剪贴板！");
        }
    }
}

// Read from stdin and return a string
fn input() -> Result<String, ()> {
    // Read from stdin
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(String::from(input.trim())), // Remove '\n' in the end
        Err(error) => Err(println!("error: {error}")),
    }
}

// Get a change list from user input
fn get_change_list(len: usize) -> Option<Vec<usize>> {
    // Previous section
    let Ok(input) = input() else {
        return None;
    };
    if input == "b" {
        Some(Vec::new())
    }
    // Next section
    else {
        let tmp: Vec<_> = input.split(' ').collect();
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
        // let unique = std::collections::BTreeSet::from_iter(change_list);
        // let mut change_list = unique.into_iter().collect::<Vec<_>>();
        // change_list.sort_unstable();

        Some(change_list)
    }
}

// Copy the result to system clipboard, require xsel
fn export_to_clipboard(result: String) -> Result<(), ()> {
    let echo_child = Command::new("echo")
        .arg(result)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");
    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");
    let status = Command::new("xsel")
        .arg("-bi")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .status()
        .expect("Failed to start xsel process");
    if status.success() {
        return Ok(());
    }
    Err(())
}
