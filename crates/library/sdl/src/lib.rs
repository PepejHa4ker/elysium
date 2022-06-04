#![feature(pointer_byte_offsets)]

//! Convenience wrapper around `elysium_dl::Library` for SDL methods.

use elysium_dl::Library;
use std::fmt;

/// The SDL library.
pub struct Sdl {
    library: Library,
}

impl Sdl {
    /// Load SDL, specifically `libSDL2-2.0.so.0`.
    #[inline]
    pub fn open() -> Option<Self> {
        let library = Library::open("libSDL2-2.0.so.0\0")?;

        Some(Self { library })
    }

    /// Obtains the absolute address from `JMP /4` located at the `SDL_GL_SwapWindow` symbol.
    ///
    /// ```asm
    /// 0000000000037a60 <SDL_GL_SwapWindow>:
    ///    37a60: ff 25 ca fc 32 00     jmp    *0x32fcca(%rip)
    /// ```
    #[inline]
    pub unsafe fn swap_window(&self) -> Option<*const ()> {
        frosting::println!("resolving absolute address of `SDL_GL_SwapWindow` from JMP /4");

        let symbol = self.library.symbol("SDL_GL_SwapWindow\0")?;
        let base = symbol.as_ptr();
        let relative = base.byte_add(2).cast::<i32>().read() as isize;
        let address = elysium_mem::to_absolute(base, relative, 6);

        Some(address)
    }

    /// Obtains the absolute address from the `JMP /4` located at the `SDL_PollEvent` symbol.
    ///
    /// ```asm
    /// 0000000000035eb0 <SDL_PollEvent>:
    ///     35eb0: ff 25 b2 0a 33 00     jmp    *0x330ab2(%rip)
    /// ```
    #[inline]
    pub unsafe fn poll_event(&self) -> Option<*const ()> {
        frosting::println!("resolving absolute address of `SDL_PollEvent` from JMP /4");

        let symbol = self.library.symbol("SDL_PollEvent\0")?;
        let base = symbol.as_ptr();
        let relative = base.byte_add(2).cast::<i32>().read() as isize;
        let address = elysium_mem::to_absolute(base, relative, 6);

        Some(address)
    }
}

impl fmt::Debug for Sdl {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.library, fmt)
    }
}
