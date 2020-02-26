use std::cell::*;
use std::rc::*;

pub struct WeakVec<T: ?Sized> {

    vec: Vec<Weak<RefCell<T>>>
}

impl<T: ?Sized> WeakVec<T> {

    pub fn new() -> Self {
        Self {
            vec: Vec::new()
        }
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(initial_capacity)
        }
    }

    pub fn push(&mut self, element: Weak<RefCell<T>>) {
        self.vec.push(element);
    }

    pub fn for_each_cell<F: Fn(Rc<RefCell<T>>) -> bool>(&mut self, closure: F) {
        self.vec.drain_filter(|weak_cell| {
            match weak_cell.upgrade() {
                Some(cell) => {
                    closure(cell)
                }, None => {
                    true
                }
            }
        });
    }

    pub fn for_each<F: Fn(&T) -> bool>(&mut self, closure: F) {
        self.for_each_cell(|cell| {
            let borrowed = cell.borrow();
            closure(&borrowed)
        });
    }

    pub fn for_each_mut<F: Fn(&mut T) -> bool>(&mut self, closure: F) {
        self.for_each_cell(|cell| {
            let mut borrowed = cell.borrow_mut();
            closure(&mut borrowed)
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_for_each() {

        let mut vec = WeakVec::new();

        let persistent1 = Rc::new(RefCell::new(1));
        let vanish1 = Rc::new(RefCell::new(2));
        let persistent2 = Rc::new(RefCell::new(4));
        let vanish2 = Rc::new(RefCell::new(8));

        vec.push(Rc::downgrade(&persistent1));
        vec.push(Rc::downgrade(&vanish1));
        vec.push(Rc::downgrade(&persistent2));
        vec.push(Rc::downgrade(&vanish2));

        let sum = Cell::new(0);
        vec.for_each(|number| {
            sum.set(sum.get() + number);
            *number == 4
        });
        assert_eq!(15, sum.get());

        let sum = Cell::new(0);
        vec.for_each(|number| {
            sum.set(sum.get() + number);
            false
        });
        assert_eq!(11, sum.get());

        drop(vanish1);
        drop(vanish2);

        let sum = Cell::new(0);
        vec.for_each(|number| {
            sum.set(sum.get() + number);
            false
        });
        assert_eq!(1, sum.get());
    }

    #[test]
    fn test_for_each_mut() {

        let mut vec = WeakVec::with_capacity(2);

        let persistent1 = Rc::new(RefCell::new("a".to_string()));
        let vanish1 = Rc::new(RefCell::new("b".to_string()));
        let persistent2 = Rc::new(RefCell::new("c".to_string()));
        let vanish2 = Rc::new(RefCell::new("d".to_string()));

        vec.push(Rc::downgrade(&persistent1));
        vec.push(Rc::downgrade(&vanish1));
        vec.push(Rc::downgrade(&persistent2));
        vec.push(Rc::downgrade(&vanish2));

        let test_string = RefCell::new("".to_string());
        vec.for_each_mut(|text| {
            text.push('e');
            test_string.borrow_mut().push_str(text);
            text == "ce"
        });
        assert_eq!("aebecede".to_string(), *test_string.borrow());

        let test_string = RefCell::new("".to_string());
        vec.for_each_mut(|text| {
            text.push('e');
            test_string.borrow_mut().push_str(text);
            false
        });
        assert_eq!("aeebeedee".to_string(), *test_string.borrow());

        drop(vanish1);
        drop(vanish2);

        let test_string = RefCell::new("".to_string());
        vec.for_each_mut(|text| {
            text.push('e');
            test_string.borrow_mut().push_str(text);
            false
        });
        assert_eq!("aeee".to_string(), *test_string.borrow());
    }
}