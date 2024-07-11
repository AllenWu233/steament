use super::{item::Item, section::Section};
use crate::template::TEMPLATE;
use crate::utils::tools::{export_to_clipboard, get_change_list, input};
use crate::utils::value::TAIL;

use std::{fs, io, io::Write};

pub struct Survey {
    game: String,
    sections: Vec<Section>,
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

    // Create a Survey from content of template.txt
    pub fn init_from_default_template() -> Self {
        Self::init_from_string(String::from(TEMPLATE))
    }

    // Create a Survey from content of template.txt like text
    fn init_from_string(s: String) -> Self {
        let messages: Vec<_> = s.lines().collect();
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

    // Read comment template from specific file to create a Survey
    pub fn _init_from_file(filename: &str) -> Self {
        let file_content = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("File {} not existed!", filename));
        Self::init_from_string(file_content)
    }

    pub fn start(&mut self) {
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
            let section = &mut self.sections[i];
            print!("\n{}", section.get_result(false));
            println!(
                "# 输入若干个序号或区间，用单个空格分隔。如：1 3-4 6\n# 输入b返回上一步。请选择："
            );
            let _ = io::stdout().flush();
            match get_change_list(section.items.len()) {
                // Previous
                Some(change_list) if change_list.is_empty() => {
                    if i == 0 {
                        println!("# 已在第一个部分！")
                    } else {
                        i -= 1;
                    }
                }
                // Next
                Some(change_list) => {
                    section.change_checkboxes(change_list);
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
