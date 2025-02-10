use rust_utils::add; // 替换 `my_crate` 为你的 crate 名字

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
