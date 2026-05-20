use browser_protocol::{browser, page};

#[test]
fn test_get_version_builder() {
    let params = browser::GetVersionParams::builder().build();
    assert_eq!(browser::GetVersionParams::METHOD, "Browser.getVersion");
    
    let serialized = serde_json::to_string(&params).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_navigate_builder() {
    let params = page::NavigateParams::builder()
        .url("https://google.com")
        .transitionType(page::TransitionType::Typed)
        .build();
        
    assert_eq!(params.url(), "https://google.com");
    assert_eq!(params.transitionType(), Some(&page::TransitionType::Typed));
    assert_eq!(params.frameId(), None);
    
    let serialized = serde_json::to_string(&params).unwrap();
    println!("Serialized JSON: {}", serialized);
    assert!(serialized.contains(r#""url":"https://google.com""#));
    assert!(serialized.contains(r#""transitionType":"typed""#));
    assert!(!serialized.contains("frameId"));
}
