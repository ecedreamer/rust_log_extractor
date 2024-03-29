use rust_log_extractor::extractor::extractor::{Matched, parse_info};

#[test]
fn test_parse_info() {
    let test_string_1 = "[INFO] User alice logged in from IP address 192.168.1.10.";
    let test_pattern_1 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    let parsed_info = parse_info(test_string_1, test_pattern_1).unwrap();
    assert_eq!(parsed_info.len(), 3);
}
