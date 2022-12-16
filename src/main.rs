use parser::{Parser};

mod parser;

fn main() {
    let mut parser = Parser::new("large_data.json");

    let root = parser.parse_value().unwrap();

    let barnaul = &root[78];

    println!(
        "{} at {}", 
        barnaul["name"], 
        barnaul["geolocation"]["coordinates"]
    );
}
