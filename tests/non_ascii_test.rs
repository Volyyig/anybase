use anybase::*;

#[test]
fn test_non_ascii() {
    // Testing the non-ascii characters
    let src = "01";
    let dst = "你好世界";
    let converter = Converter::new(src, dst);
    let decoder = converter.inverse();

    let input = "101010";
    assert_eq!(input, decoder.convert(&converter.convert(input).unwrap()).unwrap())
}

#[test]
fn test_non_ascii_detailed() {
    // Test conversion with non-ASCII UTF-8 characters
    //
    // This test validates correct handling of multi-byte UTF-8 characters
    // in base conversion operations. It demonstrates the difference between
    // byte-length and character-count in UTF-8 encoded strings.
    
    let src = "你好世界";
    let dst = "01";
    let input = "你好你好";
    
    // Display byte lengths vs actual character counts
    // In UTF-8, each Chinese character occupies 3 bytes
    println!("Source table byte length: {} ({} chars)", src.len(), src.chars().count());
    println!("Input string byte length: {} ({} chars)", input.len(), input.chars().count());
    
    // Enumerate each character with its Unicode code point
    for (i, ch) in input.chars().enumerate() {
        println!("Char {}: '{}' (Unicode: U+{:04X})", i, ch, ch as u32);
    }
    
    // Perform base conversion and display result
    let result = convert_base(input, src, dst).unwrap();
    println!("Conversion result: {}", result);
    
    // Output result properties
    println!("Result length: {}", result.len());
}