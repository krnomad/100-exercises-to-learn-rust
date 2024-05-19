mod helpers {
// "TODO: `use` 문을 추가하거나 사용하여 이 코드를 컴파일 할 수 있게 만드세요."
// "`Ticket` 구조체를 참조하기 위한 적절한 경로."

    fn create_todo_ticket(title: String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".into())
    }
}

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
