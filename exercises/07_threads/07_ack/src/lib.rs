use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// "예상 스키마를 이해하기 위해 테스트를 참조하십시오."
pub enum Command {
    Insert { todo!() },
    Get { todo!() }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// "TODO: 예상대로 들어오는 명령을 처리하십시오."
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {}) => {
                todo!()
            }
            Ok(Command::Get {
                todo!()
            }) => {
                todo!()
            }
            Err(_) => {
// "더 이상 발신자가 없으므로, 안전하게 중단할 수 있습니다."
// "그리고 서버를 종료하십시오."
                break
            },
        }
    }
}
