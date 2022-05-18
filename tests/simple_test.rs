extern crate biscuit_quote;
use biscuit_quote::block;

#[test]
fn it_works() {
    let toto = block!(r#"fact("test");"#, my_key = "my_value");
    dbg!(toto);
    panic!("no");
}

#[test]
fn it_works_with_2_parameters() {
    let toto = block!(r#"fact("test");"#, my_key = "my_value", my_key2 = 42);
    dbg!(toto);
    panic!("no");
}

#[test]
fn it_works_trailing_comma() {
    let toto = block!(r#"fact("test");"#, my_key = "my_value",);
    dbg!(toto);
    panic!("no");
}
