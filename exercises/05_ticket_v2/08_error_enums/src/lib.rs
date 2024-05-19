// "TODO: 제목 오류용과 설명 오류용 두 가지 변형을 사용하세요."
// "각 변형은 무엇이 정확하게 잘못되었는지 설명하는 문자열을 포함해야 합니다."
// "당신은 또한 `Ticket::new`의 구현을 업데이트해야 할 것입니다."
enum TicketNewError {}

// "TODO: 제목이 유효하지 않을 때 `easy_ticket`은 오류 메시지를 사용하여 패닉 상태가 되어야 합니다."
// "`TicketNewError` 열거형의 관련 변형에 저장됩니다."
// "설명이 유효하지 않을 때는 대신 기본 설명을 사용해야 합니다:"
// "설명이 제공되지 않았습니다."
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    todo!()
}

#[derive(Debug, PartialEq)]
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
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 characters".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 characters".to_string());
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
}
