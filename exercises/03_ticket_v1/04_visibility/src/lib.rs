mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 characters");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 characters");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// "TODO : **예외적으로**, 당신은 `ticket` 모듈과 `tests` 모듈을 모두 수정하게 될 것입니다."
// "이 연습에서."
#[cfg(test)]
mod tests {
// "TODO: 컴파일러를 제거하기 위해 부모 모듈에 필요한 `pub` 수정자를 추가하십시오."
// "아래의 use 문에 관한 오류들."
    use super::ticket::Ticket;

// "그러나 주의하세요! 당신이 변경한 후에 이 함수가 컴파일되는 것을 원하지 않습니다."
// "사용 구문을 컴파일할 수 있게 하는 가시성!"
// "실제로 컴파일되지 않는다는 것을 확인한 후에 주석 처리하십시오."
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

// "이 연습을 실행하려고 할 때 이 오류를 보게 될 것입니다:"
// You didn't provide any text to translate. Please write down the sentence you want to translate in the comment section.
// "오류[E0616]: 구조체 `encapsulation::ticket::Ticket`의 필드 `description`은 비공개입니다."
// "|"
// "| assert_eq!(ticket.description, "A description");"
// "|                         ^^^^^^^^^^^^^^^^^^"
// 댓글을 입력하신 내용이 없습니다. 번역하려는 영어 댓글을 입력해 주세요.
// "TODO: 아래의 것이 컴파일되지 않음을 확인한 후에,"
// "다음 연습으로 넘어가기 위해 줄을 주석 처리하세요!"
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
// "이것도 불가능해야 하며, 위에서 발생한 오류와 유사한 오류가 발생해야 합니다."
// "(오류가 있는 행을 주석 처리한 후에만 컴파일 오류가 발생합니다)"
// "이전 테스트에서 - 다음 컴파일 단계!)"
// It seems you forgot to provide any text to translate. Please try again.
// "이것은 `Ticket::new` 이제 `Ticket` 인스턴스를 얻는 유일한 방법임을 증명합니다."
// "불법적인 제목이나 설명으로 티켓을 만드는 것은 불가능합니다!"
// You didn't provide any text to translate. Please provide the text you want translated into Korean.
// "TODO: 아래 내용이 컴파일되지 않는 것을 확인한 후에,"
// "다음 연습으로 넘어가려면 줄들을 주석 처리하세요!"
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
