use ley::Ley;

#[test]
fn parse_report() {
    let ley = Ley::parse(include_str!("report.ley")).unwrap();
    println!("{:#?}", ley)
}