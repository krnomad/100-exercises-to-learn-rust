enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
// "TODO: `radius` 메소드를 사용하여 구현하십시오"
// "`if let` 또는 `let/else` 중 하나."
    pub fn radius(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}
