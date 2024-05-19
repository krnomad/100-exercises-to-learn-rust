use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// "TODO: 클라이언트 구현을 자세히 설계하십시오."
pub struct TicketStoreClient {}

impl TicketStoreClient {
// "단순화를 위해 모든 오류에 대해 자유롭게 패닉하십시오."
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        todo!()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    todo!()
}

// "더 이상 공개되지 않습니다! 이제 이것은 라이브러리의 내부 세부사항이 됩니다."
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
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
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
// "더 이상 보내는 사람이 없으므로, 안전하게 중단할 수 있습니다."
// "그리고 서버를 종료하세요."
                break;
            }
        }
    }
}
