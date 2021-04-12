use crate::string::enquote;

#[test]
fn test_enquote() {
    assert_eq!(enquote("hello"), r#""hello""#);
    assert_eq!(enquote(r#"hello "there""#), r#""hello \"there\"""#);
    assert_eq!(enquote(""), r#""""#);
}
