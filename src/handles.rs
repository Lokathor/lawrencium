use super::*;

/// Rusty version of `DECLARE_HANDLE` from `ntdef.h`
///
/// In C, it would make, `struct Foo__;` and `typedef struct Foo__ *name;`.
/// Sadly, pasting together idents to make a new ident has to be done as a
/// proc-macro in Rust, and it's not even in core, so we have this dumb thing
/// where you name it twice. Oh well.
macro_rules! declare_handle {
  ($fake:ident, $real:ident) => {
    #[repr(transparent)]
    #[doc(hidden)]
    pub struct $fake {
      _unused: u8,
    }
    pub type $real = *mut $fake;
  };
}

declare_handle!(HINSTANCE__, HINSTANCE);
declare_handle!(HICON__, HICON);
declare_handle!(HDC__, HDC);
declare_handle!(HGLRC__, HGLRC);
declare_handle!(HWND__, HWND);
declare_handle!(HBRUSH__, HBRUSH);
declare_handle!(HMENU__, HMENU);
