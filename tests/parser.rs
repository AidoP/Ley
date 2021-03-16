use ley::{Parser, Ley};

#[test]
fn parse_report() {
    Parser::new(include_str!("report.ley")).parse().unwrap();
}