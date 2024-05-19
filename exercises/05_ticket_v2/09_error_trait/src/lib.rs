// "TODO: `TicketNewError` enum에 대해 `Debug`, `Display` 및 `Error`를 구현하세요."
// "`Display`를 구현할 때, Rust의 표준 라이브러리에서 `write!` 매크로를 사용하고 싶을 수 있습니다."
// "`std::fmt` 모듈에 대한 문서는 시작하고 예제를 찾는 좋은 장소입니다:"
// "https://doc.rust-lang.org/std/fmt/index.html#write"

enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// "TODO: 제목이 유효하지 않을 때 `easy_ticket`은 에러 메시지를 사용해 패닉 상태가 되어야 합니다."
// "`TicketNewError` 열거형의 관련된 변형 내에 저장됩니다."
// "설명이 유효하지 않을 때는 대신 기본 설명을 사용해야 합니다:"
// "설명이 제공되지 않았습니다."
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    todo!()
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 characters".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 characters".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    use static_assertions::assert_impl_one;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    fn display_is_correctly_implemented() {
        let ticket = Ticket::new("".into(), valid_description(), Status::ToDo);
        assert_eq!(format!("{}", ticket.unwrap_err()), "Title cannot be empty");
    }

    assert_impl_one!(TicketNewError: std::error::Error);
}
