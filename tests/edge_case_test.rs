use anybase::*;

#[test]
fn test_zero_conversions() {
    assert_eq!(
        convert_base("0", "0123456789", "0123456789").unwrap(),
        "0"
    );
}

#[test]
fn test_single_char_tables() {
    assert_eq!(
        convert_base("1111", "01", "01").unwrap(),
        "1111"
    );
}

#[test]
fn test_same_source_destination() {
    assert_eq!(
        convert_base("abc", "abc", "abc").unwrap(),
        "bc"
    );
}