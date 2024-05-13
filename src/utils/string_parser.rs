pub fn is_digit(s: &String) -> bool {
    if s.is_empty() {
        return false
    }

    for char in s.chars() {
        if !char.is_digit(10) {
            return false;
        }
    }

    true
}