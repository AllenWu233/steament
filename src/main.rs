mod survey;

use survey::Item;
use survey::Section;
use survey::Survey;

fn main() {
    let section1 = Section::new(
        String::from("大小"),
        vec![
            Item::new(String::from("百兆小游戏")),
            Item::new(String::from("1-5 G")),
            Item::new(String::from("5-10 G")),
            Item::new(String::from("10-20 G")),
            Item::new(String::from("20-50 G")),
            Item::new(String::from("100-200 G")),
            Item::new(String::from("GMOD")),
        ],
    );

    // let section1 = Section::new(
    //     String::from(""),
    //     vec![
    //         Item::new(String::from("")),
    //         Item::new(String::from("")),
    //         Item::new(String::from("")),
    //         Item::new(String::from("")),
    //         Item::new(String::from("")),
    //         Item::new(String::from("")),
    //     ],
    // );

    // let s = Survey::new(vec![section1]);
    section1.print_section(true);
}
