struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
// "TODO: `new` 함수를 구현하시오."
// "다음의 요구사항이 충족되어야 합니다:"
// "- 오직 `To-Do`, `진행 중`, 그리고 `완료` 상태만 허용됩니다."
// "- `title`과 `description` 필드는 비워 두면 안 됩니다."
// "- `title`은 최대 50 바이트까지 가능합니다."
// "- `description`은 최대 500 바이트까지 여야 합니다."
// "요구 사항이 충족되지 않는 경우 메소드는 패닉 상태가 되어야 합니다."
// You didn't provide the English comment you want to be translated into Korean. Please provide the comment.
// "당신은 이전의 연습에서 배운 것을 사용해야 합니다."
// "일부 `String` 메소드와 함께 사용하세요. Rust의 표준 라이브러리 문서를 참조하세요."
// "가장 적합한 옵션을 찾기 위해서 -> https://doc.rust-lang.org/std/string/struct.String.html"
    fn new(title: String, description: String, status: String) -> Self {
        todo!();
        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 characters")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
