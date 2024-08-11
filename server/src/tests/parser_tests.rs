use crate::scanner::numeric_parser::parse_floats;

#[test]
fn test_colloquial() {
    assert_eq!(
        parse_floats("one million two thousand and thirty six"),
        Some(1_002_036f64)
    );
    assert_eq!(
        parse_floats("four hundred and seventy six thousand five hundred and sixty two"),
        Some(476562f64)
    );
}

#[test]
fn test_digitwise() {
    assert_eq!(parse_floats("one two three four"), Some(1234f64));
}

#[test]
fn test_year_style() {
    assert_eq!(parse_floats("twenty nineteen"), Some(2019f64));
}

#[test]
fn test_doubles() {
    assert_eq!(
        parse_floats("twenty nineteen point three two"),
        Some(2019.32)
    )
}
