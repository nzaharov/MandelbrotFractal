use rsa_course_project::{ImageSize};
use std::str::FromStr;

#[test]
fn test_size_parse() {
    let dim = "640x320";
    let parsed = ImageSize::from_str(&dim).unwrap();

    assert_eq!(parsed.width, 640_u32);
    assert_eq!(parsed.height, 320_u32);
}

#[test]
fn test_size_invalid() {
    let dim = "320";
    match ImageSize::from_str(&dim) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    };
}
