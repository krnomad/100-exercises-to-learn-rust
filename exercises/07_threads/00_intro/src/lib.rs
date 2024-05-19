fn intro() -> &'static str {
// "TODO: 나를 수정해주세요 👇"
    "I'm ready to _!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(
            intro(),
            "I'm ready to build a concurrent ticket management system!"
        );
    }
}
