fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let args = tokenize(line);
        println!("{:?}", args);
    }
}

fn tokenize(line: String) -> Vec<String> {
    let mut buffer: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();
    for c in line.chars() {
        if c.is_whitespace() && !buffer.is_empty() {
            tokens.push(buffer.iter().collect());
            Vec::clear(&mut buffer);
        } else if !c.is_whitespace() {
            buffer.push(c);
        }
    }
    if !buffer.is_empty() {
        tokens.push(buffer.iter().collect());
    }
    tokens
}
