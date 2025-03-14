// libsm64 complains if these don't exist

#[no_mangle]
pub extern "C" fn sprintf() {}

#[no_mangle]
pub extern "C" fn stop_sound(_: u32, _: *const f32) {}

#[no_mangle]
pub extern "C" fn stop_background_music() {}

#[no_mangle]
pub extern "C" fn fadeout_background_music() {}

#[no_mangle]
pub extern "C" fn fprintf() {}

// req'd w/ size optimization????

#[no_mangle]
pub extern "C" fn __small_sprintf() {}

#[no_mangle]
pub extern "C" fn siprintf() {}

#[no_mangle]
pub extern "C" fn fiprintf() {}
