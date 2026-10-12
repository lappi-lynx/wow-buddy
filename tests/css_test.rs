#[test]
fn test_css_embedded() {
    let css = wow_buddy::styles::APP_CSS;
    assert!(!css.is_empty());
    assert!(css.contains("tailwindcss"));
    assert!(css.contains("bg-slate-950"));
}
