mod sys {
    extern "C" {
        pub fn lcd_println(text: *const i8);
    }
}

#[no_mangle]
pub extern "C" fn arduino_setup() {
    println!("Hello, world!");

    lcd_println("Rust!!!");
}

pub fn lcd_println(text: &str) {
    let cstr = std::ffi::CString::new(text).unwrap();

    unsafe { sys::lcd_println(cstr.as_ptr()) };
}

#[no_mangle]
extern "C" fn arduino_loop() {}
