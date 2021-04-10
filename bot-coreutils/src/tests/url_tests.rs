use crate::url::*;

#[test]
fn it_returns_the_domain_name() {
    assert_eq!(
        get_domain_for_url("https://domain.com/sub/sub"),
        Some("domain.com".to_string())
    );
    assert_eq!(
        get_domain_for_url("other-domain.com"),
        Some("other-domain.com".to_string())
    );
    assert_eq!(get_domain_for_url("Invalid URL"), None);
    assert_eq!(get_domain_for_url("file:////what/a/file.txt"), None);
    assert_eq!(
        get_domain_for_url("https://www.domain.com/sub",),
        Some("domain.com".to_string())
    );
}
