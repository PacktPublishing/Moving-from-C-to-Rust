#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        SDL_Init(0);
        let window = SDL_CreateWindow(b"Hello World\0".as_ptr() as *const i8, 0, 0, 320, 200, 0);
        SDL_Delay(2000);
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
