use anybase::*;

#[test]
fn test_bidirectional_conversion() {
    let test_cases = vec![
        // (input, src_table, dst_table)
        ("ff", "0123456789abcdef", "01234567"),
        ("377", "01234567", "0123456789abcdef"),
        (
            "12345",
            "0123456789",
            "0123456789abcdefghijklmnopqrstuvwxyz",
        ),
        ("9ix", "0123456789abcdefghijklmnopqrstuvwxyz", "0123456789"),
        ("zzzz", "0123456789abcdefghijklmnopqrstuvwxyz", "0123456789"),
        ("0", "0123456789abcdef", "01234567"),
        ("0", "0123456789", "0123456789abcdefghijklmnopqrstuvwxyz"),
        ("abc", "abcdefghijklmnopqrstuvwxyz", "0123456789"),
        ("777", "01234567", "0123456789"),
        ("hello", "abcdefghijklmnopqrstuvwxyz", "0123456789abcdef"),
        ("rust", "abcdefghijklmnopqrstuvwxyz", "0123456789"),
        ("abc", "abcdefghijklmnopqrstuvwxyz", "0123456789"), // 带前导零的情况
    ];

    for (input, src_table, dst_table) in test_cases {
        // 正向转换: src_base -> dst_base
        let forward_result = convert_base(input, src_table, dst_table).expect(&format!(
            "Failed to convert {} from {} to {}",
            input, src_table, dst_table
        ));

        // 反向转换: dst_base -> src_base
        let backward_result = convert_base(&forward_result, dst_table, src_table).expect(&format!(
            "Failed to convert back {} from {} to {}",
            forward_result, dst_table, src_table
        ));

        // 特殊处理：检查是否因为前导零导致的问题
        // 如果原输入以0开头，则不能简单比较字符串，因为转换会去掉前导零
        let expected = if input.chars().count() > 1
            && let Some(ch) = src_table.chars().next()
        {
            input.trim_start_matches(ch)
        } else {
            &input
        };

        assert_eq!(
            expected, backward_result,
            "Bidirectional conversion failed: {} -> {} -> {} (expected {})",
            input, forward_result, backward_result, expected
        );
    }

    // 单独测试全零的情况
    let all_zero_cases = vec![
        ("0", "0123456789", "0123456789abcdef"),
        ("000", "0123456789", "0123456789abcdef"),
        ("0000", "0123456789abcdef", "01234567"),
    ];

    for (input, src_table, dst_table) in all_zero_cases {
        let forward_result = convert_base(input, src_table, dst_table).expect(&format!(
            "Failed to convert {} from {} to {}",
            input, src_table, dst_table
        ));
        let backward_result = convert_base(&forward_result, dst_table, src_table).expect(&format!(
            "Failed to convert back {} from {} to {}",
            forward_result, dst_table, src_table
        ));

        // 全零情况下应该都返回单个"0"
        assert_eq!("0", forward_result);
        assert_eq!("0", backward_result);
    }
}
