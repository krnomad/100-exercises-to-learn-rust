// "/ `n`이 짝수이면 `true`를 반환하고, 그렇지 않으면 `false`를 반환하십시오."
fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
