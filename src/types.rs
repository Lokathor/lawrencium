use super::*;

/// Lets us write C-style type aliases.
///
/// In C a type alias is written as `typedef old new;`, but in Rust we usually
/// write it `type new = old;`. This lets us have the C-style type aliases in
/// our source file so it's easier to look at the headers and then look here and
/// understand that they're the same.
macro_rules! typedef {
  ($(#[$m:meta])* $base:ty, $new:ident) => {
    $(#[$m])*
    pub type $new = $base;
  };
}

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

pub type FARPROC = *mut c_void;
pub type NEARPROC = *mut c_void;
pub type PROC = *mut c_void;

typedef!(*mut c_void, HANDLE);
typedef!(*mut c_void, PVOID);
typedef!(*mut HANDLE, PHANDLE);
typedef!(c_long, LONG);
typedef!(c_uint, UINT);
typedef!(c_uint, UINT32);
typedef!(c_ulong, DWORD);
typedef!(c_ulong, ULONG);
typedef!(c_ushort, WORD);
typedef!(HICON, HCURSOR);
typedef!(HINSTANCE, HMODULE);
typedef!(isize, INT_PTR);
typedef!(isize, LONG_PTR);
typedef!(LONG_PTR, LPARAM);
typedef!(LONG_PTR, LRESULT);
typedef!(UINT_PTR, WPARAM);
typedef!(ULONG_PTR, DWORD_PTR);
typedef!(usize, UINT_PTR);
typedef!(usize, ULONG_PTR);
typedef!(*const WCHAR, LPCWSTR);
typedef!(u16, WCHAR);
typedef!(*mut RECT, PRECT);
typedef!(*mut RECT, LPRECT);
typedef!(*mut POINT, PPOINT);
typedef!(*mut POINT, LPPOINT);
typedef!(*const CHAR, LPCSTR);
typedef!(c_char, CHAR);
typedef!(c_int, BOOL);
typedef!(c_uchar, BYTE);
typedef!(*mut MSG, LPMSG);
typedef!(WORD, ATOM);
typedef!(*mut c_void, LPVOID);
typedef!(
  Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
  WNDPROC
);
