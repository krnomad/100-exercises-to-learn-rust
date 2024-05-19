// "TODO: 이 연습 문제가 성공적으로 컴파일되기 위해서는 (파생 가능한) 특성 구현이 누락되었습니다."
// "고쳐!"
// As the comment is blank, there's nothing to translate.
// "# `디버그` 기초"
// You need to provide a specific comment to translate into Korean.
// "`Debug`는 디버깅에 적합한 Rust 타입의 표현을 반환합니다(따라서 이름이 그렇습니다)."
// "`assert_eq!`는 단언이 실패할 때 시도하기 때문에 `Ticket`이 `Debug`를 구현하도록 요구합니다."
// "비교의 양면을 터미널에 출력하세요."
// "비교되는 타입이 `Debug`을 구현하지 않는다면, 어떻게 표현할지 모릅니다!"

#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }
}
