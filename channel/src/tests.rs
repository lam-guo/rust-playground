use super::*;

#[test]
fn test_mpsc1() {
    let a = "sdfa";
    let rx = mpsc1::mpsc1(a);
    let result = rx.recv();
    assert_eq!(result.is_ok(), true);
    let event = result.unwrap();
    let expect = mpsc1::Event(Some(mpsc1::A { value: a }), None);
    debug_assert_eq!(
        event, expect,
        "testing mpsc1 recv: {:?}, {:?}",
        event, expect
    );
}
