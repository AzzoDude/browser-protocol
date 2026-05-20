use browser_protocol::{browser, page};

#[test]
fn test_get_version_default() {
    let params = browser::GetVersionParams::default();
    assert_eq!(browser::GetVersionParams::METHOD, "Browser.getVersion");
    
    let serialized = serde_json::to_string(&params).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_navigate_builder() {
    let params = page::NavigateParams::builder("https://google.com")
        .transition_type(page::TransitionType::Typed)
        .build();
        
    assert_eq!(params.url(), "https://google.com");
    assert_eq!(params.transition_type(), Some(&page::TransitionType::Typed));
    assert_eq!(params.frame_id(), None);
    
    let serialized = serde_json::to_string(&params).unwrap();
    assert!(serialized.contains(r#""url":"https://google.com""#));
    assert!(serialized.contains(r#""transitionType":"typed""#));
    assert!(!serialized.contains("frameId"));
}
