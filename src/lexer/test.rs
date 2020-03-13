use super::*;

#[test]
fn first_line() {
    // Given
    let string = "a b\nc d\nf\n\ng";
    let converter = RangeConverter::new(string);

    // When
    let line_and_pos = converter.to_line_and_pos(0..1);

    // Then
    assert_eq!(line_and_pos, (1, 1));
}

#[test]
fn second_line() {
    // Given
    let string = "a b\nc d\nf\n\ng";
    let converter = RangeConverter::new(string);

    // When
    let line_and_pos = converter.to_line_and_pos(4..5);

    // Then
    println!("{}", &string[4..5]);
    assert_eq!(line_and_pos, (2, 1));
}
