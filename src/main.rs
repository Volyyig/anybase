use anybase::convert_base;

fn main() {
    // Custom character tables (e.g. for Chinese ASCII mapping)
    let src_table = "0123456789"; // Decimal
    let dst_table = "0123456789abcdef"; // Hexadecimal

    let input = "1";

    let result = convert_base(input, src_table, dst_table);

    println!("{} (base {}) => {} (base {})",
        input, src_table.len(), result.unwrap(), dst_table.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        let src = "0123456789";
        let dst = "abcdefghijklmnopqrstuvwxyz";

        for i in "你好啊世界\u{9fff}\u{4e00}".chars() {
            let input = format!("{}", i as u32);
            let out = convert_base(&input, src, dst).unwrap();
            println!("{} -> {}", input, out);
            assert!(!out.is_empty());
        }

    }
}