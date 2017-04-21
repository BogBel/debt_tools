extern crate debt_tools;

#[test]
fn check_positive_sum() {
    assert_eq!(2, debt_tools::add(1, 1))
}

#[test]
fn check_negative_sun(){
    assert_eq!(-5, debt_tools::add(-2, -3))
}
