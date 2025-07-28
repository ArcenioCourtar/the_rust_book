use adder::add;

#[test]
fn it_adds() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
