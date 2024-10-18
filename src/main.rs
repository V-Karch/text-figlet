fn main() {
    // Get command-line arguments and skip the first argument (the program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Check if exactly one argument is provided, otherwise print usage message
    if args.len() != 1 {
        println!("Expected 1 text argument but recieved {}", args.len());
        println!("Usage: ./text-figlet <text-argument>");
        return;
    }

    let input_text: &String = &args[0];

    let standard_font: figlet_rs::FIGfont = figlet_rs::FIGfont::standard().unwrap();
    let figure: figlet_rs::FIGure<'_>  = standard_font.convert(input_text).unwrap();

    println!("{}", figure);
}
