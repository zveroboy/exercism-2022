pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(input: i32) -> i32 {
    expected_minutes_in_oven() - input
}

pub fn preparation_time_in_minutes(layers: i32) -> i32 {
    layers * 2
}

pub fn elapsed_time_in_minutes(layers: i32, elapsed: i32) -> i32 {
    preparation_time_in_minutes(layers) + elapsed
}

#[test]
fn expected_minutes_in_oven_to_be_40() {
    assert_eq!(40, expected_minutes_in_oven());
}

#[test]
fn remaining_minutes_in_oven_to_be_10() {
    assert_eq!(10, remaining_minutes_in_oven(30));
}

#[test]
fn preparation_time_in_minutes_to_be_4() {
    assert_eq!(4, preparation_time_in_minutes(2));
}

#[test]
fn elapsed_time_in_minutes_for_one_layer() {
    assert_eq!(32, elapsed_time_in_minutes(1, 30));
}

#[test]
fn elapsed_time_in_minutes_for_multiple_layers() {
    assert_eq!(16, elapsed_time_in_minutes(4, 8));
}
