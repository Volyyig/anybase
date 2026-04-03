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

#[test]
#[should_panic(expected = "src_table is empty")]
fn test_empty_src_table() {
    convert_base("123", "", "0123456789").unwrap();
}

#[test]
#[should_panic(expected = "dst_table is empty")]
fn test_empty_dst_table() {
    convert_base("123", "0123456789", "").unwrap();
}

#[test]
fn test_extreme_large_integer() {
    let large_input = "f".repeat(10000); // very large hex number
    let hex_table = "0123456789abcdef";
    let bin_table = "01"; // convert to binary

    let bin_res = convert_base(&large_input, hex_table, bin_table).unwrap();
    let hex_res = convert_base(&bin_res, bin_table, hex_table).unwrap();
    assert_eq!(large_input, hex_res);
}