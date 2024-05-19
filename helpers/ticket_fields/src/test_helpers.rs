use crate::{TicketDescription, TicketTitle};
use common::{valid_description, valid_title};

// "/ 유효한 티켓 제목을 생성하는 함수,"
// "/ 테스트 목적으로."
pub fn ticket_title() -> TicketTitle {
    valid_title().try_into().unwrap()
}

// "/ 유효한 티켓 설명을 생성하는 함수,"
// "/ 테스트 목적으로."
pub fn ticket_description() -> TicketDescription {
    valid_description().try_into().unwrap()
}
