use std::ffi::{CStr, CString};

#[repr(C)]
struct SDL_Window([u8; 0]);

#[link(name = "SDL2")]
unsafe extern "C" {
    fn SDL_Init(_: u32) -> i32;
    fn SDL_Quit();
    fn SDL_CreateWindow(
        title: *const i8,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> *mut SDL_Window;
    fn SDL_DestroyWindow(window: *mut SDL_Window);
    fn SDL_GetError() -> *const i8;
    fn SDL_Delay(ms: u32);
}

struct Window(*mut SDL_Window);

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.0);
        }
    }
}

impl Window {
    fn new(title: &str, x: i32, y: i32, w: i32, h: i32, flags: u32) -> Result<Window, String> {
        let title = CString::new(title).unwrap();
        let window_ptr = unsafe { SDL_CreateWindow(title.as_ptr(), x, y, w, h, flags) };

        if window_ptr.is_null() {
            let err = unsafe { CStr::from_ptr(SDL_GetError()).to_bytes() };
            let err = unsafe { String::from_utf8_unchecked(err.to_vec()) };
            return Err(err);
        }

        Ok(Window(window_ptr))
    }
}

fn main() {
    unsafe {
        SDL_Init(0);
    }

    let w = Window::new("Hello World", 50, 50, 320, 200, 0);

    if let Err(err) = w {
        eprintln!("Error creating window: {}", err);
    } else {
        unsafe {
            SDL_Delay(2000);
        }
    }

    unsafe {
        SDL_Quit();
    }
}
