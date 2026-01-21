use super::fund_api::parse_jsonp_response;

#[test]
fn test_parse_includes_change_percent() {
    let response = r#"jsonpgz({\"fundcode\":\"001632\",\"name\":\"测试基金\",\"gsz\":\"1.234\",\"gszzl\":\"-0.12\",\"gztime\":\"2024-10-20 15:00\"})"#;
    let fund = parse_jsonp_response(response, "001632").unwrap();
    assert_eq!(fund.change_percent, Some("-0.12".to_string()));
}
