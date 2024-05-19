fn intro() -> &'static str {
// "TODO: 나를 수정하세요 👇"
    "I'm ready to build a calculator in Rust!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }
}
