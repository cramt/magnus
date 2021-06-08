use magnus::eval_static;

#[test]
fn it_converts_to_utf8_string() {
    let _cleanup = unsafe { magnus::embed::init() };

    let val = eval_static(r#""caf\xE9".force_encoding("ISO-8859-1")"#).unwrap();
    let s = val.try_convert::<String>().unwrap();

    assert_eq!("café", s);
}
