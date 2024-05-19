// "TODO: `TicketStore::add_ticket`의 서명을 일반 유형 매개변수를 사용하도록 재작업하십시오."
// "`impl Trait` 구문보다."

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

// "`Into<Ticket>`을 `ticket`의 유형 매개변수로 사용하면, 이 메소드는 모든 유형을 허용합니다."
// "그것은 틀림없이 `티켓`으로 변환될 수 있습니다."
// "`.into()`의 문법적인 잡음을 제거함으로써, 이것은 메소드를 사용하는 것을 더 쾌적하게 만들 수 있습니다."
// "호출하는 사이트에서. 그러나 컴파일러 오류 메시지의 품질을 악화시킬 수 있습니다."
    pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
// "`add_ticket`가 인자 위치에서 `impl Trait` 구문을 사용하면 이것은 컴파일되지 않을 것입니다."
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
