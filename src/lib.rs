extern crate libc;

use libc::c_char;
use std::collections::HashMap;
use std::ffi::CStr;

pub struct StringCounter {
    elts: HashMap<String, u64>,
}

impl StringCounter {
    fn new() -> StringCounter {
        StringCounter {
            elts: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str) {
        match self.elts.get_mut(key) {
            Some(counter) => *counter = *counter + 1,
            None => {
                self.elts.insert(key.to_string(), 1); //{
                                                      // Some(counter) => ,
                                                      //None =>  //
            }
        }
    }

    fn count(&self, key: &str) -> u64 {
        match self.elts.get(key) {
            Some(counter) => *counter,
            None => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn string_counter_new() -> *mut StringCounter {
    Box::into_raw(Box::new(StringCounter::new()))
}

#[no_mangle]
pub extern "C" fn string_counter_free(ptr: *mut StringCounter) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn string_counter_insert(ptr: *mut StringCounter, key: *const c_char) {
    let sc = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key)
    };
    let key_str = key.to_str().unwrap();
    sc.insert(key_str)
}

#[no_mangle]
pub extern "C" fn string_counter_count(ptr: *mut StringCounter, key: *const c_char) -> u64 {
    let sc = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key)
    };
    let key_str = key.to_str().unwrap();
    sc.count(key_str)
}

// https://doc.rust-lang.org/reference/type-layout.html

#[repr(C)]
pub struct Foo {
    _data: u64,
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}


#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello world!");
}

// Stack allocated collections https://github.com/japaric/heapless
