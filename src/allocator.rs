use std::{
    alloc::{alloc, alloc_zeroed, dealloc, Layout},
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use log::info;

const ALIGN: usize = 8;

// thanks https://www.reddit.com/r/rust/comments/18x9nxg/comment/kg2y9ze/ :3
fn get_malloc_map() -> MutexGuard<'static, HashMap<usize, Layout>> {
    static MAP: OnceLock<Mutex<HashMap<usize, Layout>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned!")
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size, ALIGN);
    let ptr = alloc(layout);
    let mut map = get_malloc_map();
    map.insert(ptr as usize, layout);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn calloc(n: usize, size: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(n * size, ALIGN);
    let ptr = alloc_zeroed(layout);
    let mut map = get_malloc_map();
    map.insert(ptr as usize, layout);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if ptr as usize != 0 {
        let mut map = get_malloc_map();
        let layout = map
            .get(&(ptr as usize))
            .expect("That pointer does not exist!");
        dealloc(ptr, *layout);
        map.remove(&(ptr as usize));
    }
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let mut map = get_malloc_map();
    let new_layout = Layout::from_size_align_unchecked(new_size, ALIGN);

    let new_ptr = if ptr as usize == 0 {
        alloc(new_layout)
    } else {
        let layout = map
            .remove(&(ptr as usize))
            .expect("That pointer does not exist!");
        std::alloc::realloc(ptr, layout, new_size)
    };

    map.insert(new_ptr as usize, new_layout);
    new_ptr
}
