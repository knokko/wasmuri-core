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

    pub fn for_each_cell<F: FnMut(Rc<RefCell<T>>) -> bool>(&mut self, mut closure: F) {
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

    pub fn for_each<F: FnMut(&T)>(&mut self, mut closure: F) {
        self.for_each_cell(|cell| {
            let borrowed = cell.borrow();
            closure(&borrowed);
            false
        });
    }

    pub fn for_each_mut<F: FnMut(&mut T)>(&mut self, mut closure: F) {
        self.for_each_cell(|cell| {
            let mut borrowed = cell.borrow_mut();
            closure(&mut borrowed);
            false
        });
    }
}

pub struct WeakMetaVec<T: ?Sized, M> {

    pub vec: Vec<WeakMetaHandle<T,M>>
}

pub struct WeakMetaHandle<T: ?Sized, M> {

    pub weak_cell: Weak<RefCell<T>>,
    pub metadata: M
}

impl<T: ?Sized, M> WeakMetaVec<T, M> {

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

    pub fn push(&mut self, weak_cell: Weak<RefCell<T>>, metadata: M) {
        self.vec.push(WeakMetaHandle {
            weak_cell,
            metadata
        });
    }

    pub fn for_each_cell<F: FnMut(Rc<RefCell<T>>, &mut M) -> bool>(&mut self, mut closure: F) {
        self.vec.drain_filter(|handle| {
            match handle.weak_cell.upgrade() {
                Some(cell) => {
                    closure(cell, &mut handle.metadata)
                }, None => {
                    true
                }
            }
        });
    }

    pub fn for_each<F: FnMut(&T, &M)>(&mut self, mut closure: F) {
        self.for_each_cell(|cell, meta| {
            let borrowed = cell.borrow();
            closure(&borrowed, meta);
            false
        });
    }

    pub fn for_each_mut<F: FnMut(&mut T, &mut M)>(&mut self, mut closure: F) {
        self.for_each_cell(|cell, meta| {
            let mut borrowed = cell.borrow_mut();
            closure(&mut borrowed, meta);
            false
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

        let mut sum = 0;
        vec.for_each(|number| {
            sum += number;
        });
        assert_eq!(15, sum);

        drop(vanish1);
        drop(vanish2);

        let mut sum = 0;
        vec.for_each(|number| {
            sum += number;
        });
        assert_eq!(5, sum);
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

        let mut test_string = String::new();
        vec.for_each_mut(|text| {
            text.push('e');
            test_string.push_str(text);
        });
        assert_eq!("aebecede".to_string(), test_string);

        drop(vanish1);
        drop(vanish2);

        let mut test_string = String::new();
        vec.for_each_mut(|text| {
            text.push('e');
            test_string.push_str(text);
        });
        assert_eq!("aeecee".to_string(), test_string);
    }

    #[test]
    fn test_meta_for_each() {

        let mut vec = WeakMetaVec::new();

        let persistent1 = Rc::new(RefCell::new(1));
        let vanish1 = Rc::new(RefCell::new(2));
        let persistent2 = Rc::new(RefCell::new(4));
        let vanish2 = Rc::new(RefCell::new(8));

        vec.push(Rc::downgrade(&persistent1), 3);
        vec.push(Rc::downgrade(&vanish1), 4);
        vec.push(Rc::downgrade(&persistent2), 6);
        vec.push(Rc::downgrade(&vanish2), 10);

        let mut sum = 0;
        vec.for_each(|number, meta| {
            sum += number;
            assert_eq!(*meta, number + 2);
        });
        assert_eq!(15, sum);

        drop(vanish1);
        drop(vanish2);

        let mut sum = 0;
        vec.for_each(|number, meta| {
            sum += number;
            assert_eq!(*meta, number + 2);
        });
        assert_eq!(5, sum);
    }

    #[test]
    fn test_meta_for_each_mut() {

        let mut vec = WeakMetaVec::with_capacity(2);

        let persistent1 = Rc::new(RefCell::new("a".to_string()));
        let vanish1 = Rc::new(RefCell::new("b".to_string()));
        let persistent2 = Rc::new(RefCell::new("c".to_string()));
        let vanish2 = Rc::new(RefCell::new("d".to_string()));

        vec.push(Rc::downgrade(&persistent1), 'a');
        vec.push(Rc::downgrade(&vanish1), 'b');
        vec.push(Rc::downgrade(&persistent2), 'c');
        vec.push(Rc::downgrade(&vanish2), 'd');

        let mut test_string = String::new();
        vec.for_each_mut(|text, first| {
            text.push('e');
            assert_eq!(*first, text.chars().next().unwrap());
            test_string.push_str(text);
        });
        assert_eq!("aebecede".to_string(), test_string);

        let mut test_string = String::new();
        vec.for_each_mut(|text, first| {
            text.push('e');
            assert_eq!(*first, text.chars().next().unwrap());
            test_string.push_str(text);
        });
        assert_eq!("aeebeeceedee".to_string(), test_string);

        drop(vanish1);
        drop(vanish2);

        let mut test_string = String::new();
        vec.for_each_mut(|text, first| {
            text.push('e');
            assert_eq!(*first, text.chars().next().unwrap());
            test_string.push_str(text);
        });
        assert_eq!("aeeeceee".to_string(), test_string);
    }
}