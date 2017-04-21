extern crate debt_tools;

#[test]
fn check_positive_sum() {
    assert_eq!(2, debt_tools::add(1, 1))
}

#[test]
fn check_negative_sun(){
    assert_eq!(-5, debt_tools::add(-2, -3))
}

#[test]
fn check_zero_sum(){
    assert_eq!(0, debt_tools::add(0, 0))
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn check_overflow(){
    let max_val = i32::max_value();
    debt_tools::add(max_val, 1);
}
