#[macro_use]
extern crate nom;

pub mod object;

#[cfg(test)]
mod tests {

    use object;

    #[test]
    fn test_parse_load() {
        let path = "african_head.obj";
        if let Ok(parsed) = object::parse_file_to_object(path) {
            println!("{:#?}", parsed);
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_comment() {
        let to_parse = r#"v  0.000283538 -1 0.286843
v  -0.117277 -0.973564 0.306907
v  -0.382144 -0.890788 0.221243
v  -0.247144 -0.942602 0.276051
v  -0.656078 -0.718512 -0.109025
v  -0.609847 -0.786562 0.0198068
v  -0.66248 -0.632053 -0.244271
v  -0.511812 -0.845392 0.127809
v  -0.609326 -0.569868 -0.41571
# this is a comment
vt  0.833 0.667 0.000
vt  0.708 0.836 0.000
vt  0.500 0.205 0.000
vt  0.826 0.364 0.000
vt  0.801 0.303 0.000
vt  0.613 0.529 0.000
vt  0.607 0.544 0.000
vt  0.541 0.380 0.000
vt  0.538 0.372 0.000
vt  0.537 0.366 0.000
vt  0.537 0.363 0.000
vt  0.598 0.945 0.000
vt  0.524 0.902 0.000
vt  0.599 0.489 0.000
vt  0.609 0.563 0.000
vt  0.596 0.563 0.000
vt  0.612 0.559 0.000
vt  0.557 0.569 0.000
vt  0.620 0.571 0.000
vt  0.616 0.569 0.000
vt  0.618 0.569 0.000
vt  0.569 0.568 0.000
"#;

        if let Ok(parsed) = object::parse_object(to_parse) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_object_vn() {
        let to_parse = r#"vt  0.532 0.923 0.000
vt  0.535 0.917 0.000
vt  0.542 0.923 0.000
vt  0.541 0.929 0.000
vt  0.521 0.984 0.000
vt  0.521 0.996 0.000
vt  0.505 0.998 0.000
vt  0.500 0.985 0.000
vt  0.504 0.917 0.000
vt  0.507 0.910 0.000
vt  0.516 0.910 0.000
vt  0.515 0.918 0.000
"#;
        if let Ok(parsed) = object::parse_object(to_parse) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_object() {
        let to_parse = r#"v -0.000581696 -0.734665 -0.623267
v 0.000283538 -1 0.286843
v -0.117277 -0.973564 0.306907
v -0.382144 -0.890788 0.221243
v -0.247144 -0.942602 0.276051
v -0.656078 -0.718512 -0.109025
v -0.609847 -0.786562 0.0198068
v -0.66248 -0.632053 -0.244271
v -0.511812 -0.845392 0.127809
v -0.609326 -0.569868 -0.41571
"#;
        if let Ok(parsed) = object::parse_object(to_parse) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
