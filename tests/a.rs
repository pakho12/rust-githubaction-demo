use publish_crate_with_github_actions_demo::utils::add;

#[test]
fn feature() {
    let sum = add(1, 1);
    assert_eq!(sum, 2);
}
