// "TODO: 우리가 방금 소유권에 대해 배운 것을 기반으로, 불변의 참조처럼 들립니다"
// "우리의 액세서 메소드에 잘 맞습니다."
// "`Ticket`의 접근자 메소드의 기존 구현을 참조로 변경하십시오."
// "`self`를 소유권을 가지는 것이 아니라, 인자로 사용합니다."

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
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

    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
// "요청한 대로 서명을 변경하면 이것은 컴파일 될 것입니다:"
// "우리는 이런 메서드들을 차례대로 호출할 수 있습니다 왜냐하면 그들은 `self`를 빌려오기 때문입니다."
// "그것을 소유하는 것보다는."
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
