fn intro() -> &'static str {
// "TODO: 고쳐주세요 👇"
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to refine the `Ticket` type!");
    }
}
