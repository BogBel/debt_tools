pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn check_positive_sum() {
    assert_eq!(2, add(1, 1))
}

#[test]
fn check_negative_sun(){
    assert_eq!(-5, add(-2, -3))
}

#[test]
fn check_zero_sum(){
    assert_eq!(0, add(0, 0))
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn check_overflow(){
    let max_val = i32::max_value();
    add(max_val, 1);
}
