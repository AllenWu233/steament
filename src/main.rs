mod survey;

fn main() {
    let mut s = survey::Survey::init_from_file("comment.txt");
    s.run();
}
