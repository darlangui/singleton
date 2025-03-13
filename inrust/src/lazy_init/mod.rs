// safe singleton
pub fn change (global_state: &mut u32) {
    *global_state += 1;
}

// using lazy_static
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref ARRAY: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

pub fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

// using mutex

pub static OTHER_ARRAY: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub fn other_do_a_call() {
    OTHER_ARRAY.lock().unwrap().push(1);
}