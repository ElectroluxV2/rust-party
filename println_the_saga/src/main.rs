fn main() {
    assert_eq!(format!("{} days", 69), "69 days");
    assert_eq!(format!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob"), "Alice, this is Bob. Bob, this is Alice.");
    assert_eq!(format!("Let me see, today we have: {}, {0} and {subject}.", subject = "Math"), "Let me see, today we have: Math, Math and Math.");
    assert_eq!(format!("{:b}", 2137), "100001011001");
    assert_eq!(format!("{:0width$}", number = 1, width = 6), "000001");
    assert_eq!(format!("{:05.2}€", 5.2), "05.20€");
}
