// "TODO: `Ticket`의 접근자 메소드를 다시 구현하십시오. 이번에는 `&String`이 아닌 `&str`을 반환하십시오."

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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
// "예상한 반환 타입을 사용했는지 확인하는 몇몇의 어두운 마법"
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
