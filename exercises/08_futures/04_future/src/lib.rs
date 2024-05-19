// "! TODO: 문장들을 **재정렬**하여 코드가 컴파일 될 수 있게하세요"
// "`example` 함수에서! 변경이 허용되지 않습니다."
// "! `spawner` 함수도 `example`의 각 줄이 무엇을 하는지도 모릅니다."
// "! 필요하다면 기존의 문을 `{}` 블록으로 감쌀 수 있습니다."
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
