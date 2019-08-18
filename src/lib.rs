#![no_std]
extern crate alloc;
use no_std_compat::prelude::v1::*;

pub struct Debouncer {
    patterns: Vec<u8>
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum DebounceResult {
    NoChange,
    Pressed,
    Released
}

impl Debouncer {
    pub fn new(no_of_keys: usize) -> Debouncer {
        Debouncer{
            patterns: vec![0; no_of_keys],
        }
    }

    pub fn update(&mut self, key_no: usize, pressed: bool) -> DebounceResult
    {
        let next: u8 = if pressed {1} else {0};
        self.patterns[key_no] = self.patterns[key_no] << 1 | next;
        let mut result = DebounceResult::NoChange;
        //debounce following hackadays ultimate debouncing schema
        let mask: u8 = 0b11000111;
        let seen = self.patterns[key_no] & mask;
        if seen == 0b00000111 {
            result = DebounceResult::Pressed;
            self.patterns[key_no] = 0b1111111;
        }
        else if seen == 0b11000000 {
            result = DebounceResult::Released;
            self.patterns[key_no] = 0b0000000;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::{DebounceResult, Debouncer};
    #[test]
    fn it_works() {
        let mut db = Debouncer::new(1);
        //activate
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::Pressed);
        //deactivate
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::Released);

        //let's do noise.
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);

        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::Pressed);
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, false) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::NoChange);
        assert!(db.update(0, true) == DebounceResult::NoChange);


    }
}
