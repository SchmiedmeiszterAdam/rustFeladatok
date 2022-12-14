fn add_two_numbers(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
fn display_result(res: i32) {
    println!("{:?}", res);
}
fn main() {
    let res = add_two_numbers(5, 8);
    display_result(res);
}