use rust_githubaction_demo::utils::add;

#[test]
fn example_1_2() {
    let sum = add(1, 2);
    assert_eq!(sum, 3);
}

fn main() {
    let sum = add(1, 2);
    assert_eq!(sum, 3);
    println!("sum is: {}", sum);
}
