// "TODO: `Rc`와 `RefCell`을 사용하여 타입 `T`의 값 주변에 래퍼인 `DropTracker<T>`을 구현하십시오."
// "래핑된 값이 드롭될 때마다 공유 'usize' 카운터를 증가시킵니다."

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: todo!(),
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: todo!()) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        assert_eq!(*counter.borrow(), 2);
    }
}
