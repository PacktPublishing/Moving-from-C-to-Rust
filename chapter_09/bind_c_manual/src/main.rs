use std::ffi::c_char;

#[repr(C)]
struct SDL_Window([u8; 0]);

#[link(name = "SDL2")]
unsafe extern "C" {
    fn SDL_Init(_: u32) -> i32;
    fn SDL_Quit();
    fn SDL_CreateWindow(
        title: *const c_char,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> *mut SDL_Window;
    fn SDL_Delay(ms: u32);
    fn SDL_DestroyWindow(window: *mut SDL_Window);
}

fn main() {
    let name = c"Hello World";

    unsafe {
        SDL_Init(0);
        let window = SDL_CreateWindow(name.as_ptr(), 0, 0, 320, 200, 0);
        SDL_Delay(2000);
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
