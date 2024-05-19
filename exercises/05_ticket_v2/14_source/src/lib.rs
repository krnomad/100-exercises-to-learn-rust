use crate::status::Status;

// "가장 초기의 연습 중 하나에서 모듈을 선언하는 방법을 보았지만,"
// "우리는 그것들을 별도의 파일로 추출하는 방법을 보지 못했습니다."
// "지금 고쳐 봅시다!"
// You haven't provided any English comment. Please provide the text you need to translate.
// "가장 간단한 경우로, 추출된 모듈이 단일 파일일 때에는 이것으로 충분합니다."
// "모듈과 같은 이름의 새 파일을 만들고 모듈 내용을 그곳으로 이동하십시오."
// "모듈 파일은 모듈을 선언하는 파일과 같은 디렉토리에 위치시켜야 합니다."
// "이 경우에는 `src/lib.rs`, 따라서 `status.rs`는 `src` 디렉토리에 위치해야 합니다."
mod status;

// "TODO: 상태 문자열이 유효하지 않을 경우를 위한 `TicketNewError`에 새로운 오류 변형을 추가하세요."
// "그 변형의 오류에 `source`를 호출할 때, `None`이 아닌 `ParseStatusError`를 반환해야 합니다."

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 characters")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 characters")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

// "TODO: 상태 문자열을 `Status` 열거형으로 구문 분석하십시오."

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
