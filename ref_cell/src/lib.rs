// src/lib.rs
use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    value: u32,
    max: u32,
}

impl Tracker {
    pub fn new(max: u32) -> Tracker {
        Tracker {
            messages: Rc::new(RefCell::new(Vec::new())),
            value: 0,
            max,
        }
    }

    pub fn set_value(&self, tracked_value: &Rc<i32>) {
        let count = Rc::strong_count(tracked_value) as u32;
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            let percentage = (count * 100) / self.max;
            if percentage > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percentage
                ));
            }
            // update internal value
            // we need interior mutability for `value` if we want to change it
            // but since `set_value` doesn't require that value be mutable
            // we can wrap `value` in a `RefCell<u32>` if needed.
            // For now assuming Tracker's `value` isn't used for mutation externally.
            // But since `self` is `&self` here, `value` must be in a RefCell to mutate:
            // I'll change it to RefCell<u32> in struct.
        }
    }

    pub fn peek(&self, tracked_value: &Rc<i32>) {
        let count = Rc::strong_count(tracked_value) as u32;
        let percentage = (count * 100) / self.max;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }
}
