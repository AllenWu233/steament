pub mod constant {
    pub const EMPTY_CHECKBOX: &str = "☐";
    pub const SELECT_CHECKBOX: &str = "✓";

    // pub const TAIL: &str = "——该评论使用[url=https://github.com/AllenWu233/steament]steament[/url]生成";
    pub const TAIL: &str = "——该评论使用 steament(github/AllenWu233/steament) 生成";
}

pub mod tools {
    use std::io::{self};
    use std::process::{Command, Stdio};

    // Read from stdin and return a string
    pub fn input() -> Result<String, ()> {
        // Read from stdin
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(String::from(input.trim())), // Remove '\n' in the end
            Err(error) => Err(println!("error: {error}")),
        }
    }

    // Get a change list from user input
    pub fn get_change_list(len: usize) -> Option<Vec<usize>> {
        // Previous section
        let Ok(input) = input() else {
            return None;
        };
        if input == "b" || input == "B" {
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
    pub fn export_to_clipboard(result: String) -> Result<(), ()> {
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
}
