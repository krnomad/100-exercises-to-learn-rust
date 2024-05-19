use tokio::net::TcpListener;

// "TODO: 들어오는 TCP 연결을 받아들이는 에코 서버를 작성하십시오."
// "클라이언트에게 받은 데이터를 다시 에코합니다."
// "`echo`는 연결 처리를 완료했을 때 반환되어서는 안 되지만, 그렇게 해야 합니다"
// "계속해서 새로운 연결을 수락하십시오."
// Sorry, there is nothing to translate in your text. Please provide a specific English comment.
// "힌트: 에코 서버를 구현하기 위해서는 `tokio`의 구조체와 메소드를 의존해야 합니다."
// "특히:"
// "- 다음 들어오는 연결을 처리하기 위한 `tokio::net::TcpListener::accept`"
// "- 소켓으로부터 리더와 라이터를 얻기 위해 `tokio::net::TcpStream::split` 사용"
// "- `tokio::io::copy`는 리더에서 라이터로 데이터를 복사하는 데 사용됩니다."
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

// "요청을 보내세요"
            writer.write_all(request.as_bytes()).await.unwrap();
// "소켓의 쓰기 측을 닫아라"
            writer.shutdown().await.unwrap();

// "응답을 읽어보세요"
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
