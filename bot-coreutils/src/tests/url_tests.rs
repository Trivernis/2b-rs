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

#[test]
fn it_checks_for_image() {
    assert!(is_image("domain.com/image.png"));
    assert!(is_image("https://domain.com/image.jpeg?yo=someparam"));
    assert!(!is_image("https://domain.com"));
    assert!(!is_image("https://domain.com/file.pdf"));
    assert!(!is_image("not an url"));
}

#[test]
fn it_checks_for_video() {
    assert!(is_video("domain.com/video.mp4"));
    assert!(is_video("https://domain.com/video.webm?yo=someparam"));
    assert!(!is_video("https://domain.com"));
    assert!(!is_video("https://domain.com/file.pdf"));
    assert!(!is_video("not an url"));
}
