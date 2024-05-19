pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
// "u32의 최대값에서 멈추기 위해 포화 곱셈을 사용하세요"
// "넘쳐흐르고 감싸는 것보다"
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
