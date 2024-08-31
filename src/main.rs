use console::Term;

fn main() {
    loop {
        let term = Term::stdout();
        let line = Term::read_line(&term).unwrap();
        let parsed_line = parse_line(line);
    }
}

fn parse_line(line: String) -> String {
    println!("{}", line);
    line
}
