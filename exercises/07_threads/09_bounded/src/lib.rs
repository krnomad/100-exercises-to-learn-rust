// "TODO: 구현을 경계 채널을 사용하도록 변환하십시오."
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: todo!(),
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, todo!()> {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, todo!()> {
        todo!()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    todo!();
    std::thread::spawn(move || server(receiver));
    todo!()
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: todo!(),
    },
    Get {
        id: TicketId,
        response_channel: todo!(),
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                todo!()
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                todo!()
            }
            Err(_) => {
// "더 이상 보내는 사람이 없으므로, 우리는 안전하게 중단할 수 있습니다."
// "그리고 서버를 종료하세요."
                break;
            }
        }
    }
}
