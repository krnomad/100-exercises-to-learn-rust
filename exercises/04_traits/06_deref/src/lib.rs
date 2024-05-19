// "TODO: `title`과 `description`이 그들의 접근자 메소드를 통해 반환될 때마다 그들이"
// "정규화되어야 합니다 - 즉, 선행 및 후행 공백을 제거해야 합니다."
// "러스트의 표준 라이브러리에는 이 문제를 해결하는 방법이 있지만, 사용하진 않을 것입니다."
// "`String`에 대한 문서에서 찾으십시오."
// "그것이 어디에서 정의되었는지, 그리고 어떻게 사용하는지 알아낼 수 있나요?"

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
