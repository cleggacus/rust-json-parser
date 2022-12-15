use parser::lexer::Lexer;

mod parser;

fn print_next_token(lexer: &mut Lexer) -> bool{
    match lexer.next_token() {
        None => {
            println!("None");
            return true;
        },
        Some(token) => {
            println!("{:?}", token);
            return false;
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("data.json");

    loop {
        if print_next_token(&mut lexer) {
            break;
        }
    }
}
