fn rot13(c: char) -> char {
    if !c.is_alphabetic() {
        return c;
    }

    let numeric_value = c as u32;
    // ASCII values for 'a' and 'z'
    const Z_LOWER: u32 = 122;
    const A_LOWER: u32 = 97;
    const Z_UPPER: u32 = 90;
    const A_UPPER: u32 = 65;

    let mut a = A_LOWER;
    let mut z = Z_LOWER;
    let mut rotated_value = numeric_value + 13;

    if c.is_uppercase() {
        a = A_UPPER;
        z = Z_UPPER;
    }

    if numeric_value + 13 > z {
        let rollover = rotated_value - z;
        rotated_value = a + rollover;
    }

    std::char::from_u32(rotated_value).unwrap()
}

pub fn encrypt(input: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();

    for c in chars {
        result.push(rot13(c));
    }

    result
}

#[cfg(test)]
#[test]
fn encrypts_simple_string() {
    let input = String::from("abc");
    assert_eq!(encrypt(input), String::from("nop"))
}

#[test]
fn encrypts_uppercase() {
    let input = String::from("ABC");
    assert_eq!(encrypt(input), String::from("NOP"))
}

#[test]
fn encrypts_rollover() {
    let input = String::from("xyz");
    assert_eq!(encrypt(input), String::from("lmn"))
}

#[test]
fn ignores_non_alphabetic() {
    let input = String::from("aBc123_~!");
    assert_eq!(encrypt(input), String::from("nOp123_~!"))
}

#[test]
fn ignores_empty_string() {
    let input = String::from("");
    assert_eq!(encrypt(input), String::from(""))
}
