mod survey;
mod template;

use crate::template::TEMPLATE;

fn main() {
    let mut s = survey::Survey::init_from_string(String::from(TEMPLATE));
    s.run();
}
