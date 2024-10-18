use figlet_rs::FIGfont;

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Hello Rust");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}