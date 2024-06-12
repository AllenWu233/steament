mod survey;
mod template;

fn main() {
    // let mut s = survey::Survey::init_from_file("template.txt");
    let mut s = survey::Survey::init_from_string(String::from(template::TEMPLATE));
    s.run();
}
