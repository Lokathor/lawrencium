#![no_std]
#![cfg(windows)]
#![allow(bad_style)]

//! `lawrencium` is a minimal set of Windows API bindings.
//!
//! ## Cargo Features
//! * `struct_debug`: structs derive [`Debug`](core::fmt::Debug)
//! * `struct_eq`: structs derive [`PartialEq`](core::cmp::PartialEq) and
//!   [`Eq`](core::cmp::Eq).
//! * `struct_hash`: structs derive [`Hash`](core::hash::Hash)
//! * `struct_default`: structs implement [`Default`](core::default::Default).

use core::ffi::c_void;

macro_rules! submodule {
  ($v:vis $name:ident) => {
    mod $name;
    $v use $name::*;
  };
  ($v:vis $name:ident { $($content:tt)* }) => {
    mod $name { $($content)* }
    $v use $name::*;
  };
}

submodule!(pub types);
submodule!(pub handles);

macro_rules! define {
  ($(#[$m:meta])* $name:ident : $t:ty, $val:expr) => {
    $(#[$m])*
    #[allow(unused_parens)]
    pub const $name: $t = $val;
  };
  ($(#[$m:meta])* $name:ident, $val:expr) => {
    $(#[$m])*
    #[allow(unused_parens)]
    pub const $name: u32 = $val;
  };
}

define!(FALSE, 0);
define!(TRUE, 1);

define!(PFD_TYPE_RGBA, 0);
define!(PFD_TYPE_COLORINDEX, 1);

define!(PFD_MAIN_PLANE, 0);
define!(PFD_OVERLAY_PLANE, 1);
define!(PFD_UNDERLAY_PLANE: c_int, -1);

define!(PFD_DOUBLEBUFFER, 0x00000001);
define!(PFD_STEREO, 0x00000002);
define!(PFD_DRAW_TO_WINDOW, 0x00000004);
define!(PFD_DRAW_TO_BITMAP, 0x00000008);
define!(PFD_SUPPORT_GDI, 0x00000010);
define!(PFD_SUPPORT_OPENGL, 0x00000020);
define!(PFD_GENERIC_FORMAT, 0x00000040);
define!(PFD_NEED_PALETTE, 0x00000080);
define!(PFD_NEED_SYSTEM_PALETTE, 0x00000100);
define!(PFD_SWAP_EXCHANGE, 0x00000200);
define!(PFD_SWAP_COPY, 0x00000400);
define!(PFD_SWAP_LAYER_BUFFERS, 0x00000800);
define!(PFD_GENERIC_ACCELERATED, 0x00001000);
define!(PFD_SUPPORT_DIRECTDRAW, 0x00002000);
define!(PFD_DIRECT3D_ACCELERATED, 0x00004000);
define!(PFD_SUPPORT_COMPOSITION, 0x00008000);

define!(CS_VREDRAW, 0x0001);
define!(CS_HREDRAW, 0x0002);
define!(CS_DBLCLKS, 0x0008);
define!(CS_OWNDC, 0x0020);
define!(CS_CLASSDC, 0x0040);
define!(CS_PARENTDC, 0x0080);
define!(CS_NOCLOSE, 0x0200);
define!(CS_SAVEBITS, 0x0800);
define!(CS_BYTEALIGNCLIENT, 0x1000);
define!(CS_BYTEALIGNWINDOW, 0x2000);
define!(CS_GLOBALCLASS, 0x4000);
define!(CS_IME, 0x00010000);
define!(CS_DROPSHADOW, 0x00020000);

define!(
  #[allow(overflowing_literals)]
  CW_USEDEFAULT: c_int,
  (0x80000000 as c_int)
);

define!(GWL_WNDPROC: c_int, -4);
define!(GWL_HINSTANCE: c_int, -6);
define!(GWL_HWNDPARENT: c_int, -8);
define!(GWL_STYLE: c_int, -16);
define!(GWL_EXSTYLE: c_int, -20);
define!(GWL_USERDATA: c_int, -21);
define!(GWL_ID: c_int, -12);

define!(QS_KEY, 0x0001);
define!(QS_MOUSEMOVE, 0x0002);
define!(QS_MOUSEBUTTON, 0x0004);
define!(QS_POSTMESSAGE, 0x0008);
define!(QS_TIMER, 0x0010);
define!(QS_PAINT, 0x0020);
define!(QS_SENDMESSAGE, 0x0040);
define!(QS_HOTKEY, 0x0080);
define!(QS_ALLPOSTMESSAGE, 0x0100);
define!(QS_RAWINPUT, 0x0400);
define!(QS_TOUCH, 0x0800);
define!(QS_POINTER, 0x1000);
define!(QS_MOUSE, (QS_MOUSEMOVE | QS_MOUSEBUTTON));
define!(QS_INPUT, (QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER));
define!(
  QS_ALLEVENTS,
  (QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY)
);
define!(
  QS_ALLINPUT,
  (QS_INPUT
    | QS_POSTMESSAGE
    | QS_TIMER
    | QS_PAINT
    | QS_HOTKEY
    | QS_SENDMESSAGE)
);

define!(PM_NOREMOVE, 0x0000);
define!(PM_REMOVE, 0x0001);
define!(PM_NOYIELD, 0x0002);
define!(PM_QS_INPUT, (QS_INPUT << 16));
define!(PM_QS_POSTMESSAGE, ((QS_POSTMESSAGE | QS_HOTKEY | QS_TIMER) << 16));
define!(PM_QS_PAINT, (QS_PAINT << 16));
define!(PM_QS_SENDMESSAGE, (QS_SENDMESSAGE << 16));

define!(SIZE_RESTORED, 0);
define!(SIZE_MINIMIZED, 1);
define!(SIZE_MAXIMIZED, 2);
define!(SIZE_MAXSHOW, 3);
define!(SIZE_MAXHIDE, 4);

define!(WS_OVERLAPPED, 0x00000000);
define!(WS_POPUP, 0x80000000);
define!(WS_CHILD, 0x40000000);
define!(WS_MINIMIZE, 0x20000000);
define!(WS_VISIBLE, 0x10000000);
define!(WS_DISABLED, 0x08000000);
define!(WS_CLIPSIBLINGS, 0x04000000);
define!(WS_CLIPCHILDREN, 0x02000000);
define!(WS_MAXIMIZE, 0x01000000);
define!(WS_CAPTION, 0x00C00000);
define!(WS_BORDER, 0x00800000);
define!(WS_DLGFRAME, 0x00400000);
define!(WS_VSCROLL, 0x00200000);
define!(WS_HSCROLL, 0x00100000);
define!(WS_SYSMENU, 0x00080000);
define!(WS_THICKFRAME, 0x00040000);
define!(WS_GROUP, 0x00020000);
define!(WS_TABSTOP, 0x00010000);

define!(WS_MINIMIZEBOX, 0x00020000);
define!(WS_MAXIMIZEBOX, 0x00010000);

define!(WS_TILED, WS_OVERLAPPED);
define!(WS_ICONIC, WS_MINIMIZE);
define!(WS_SIZEBOX, WS_THICKFRAME);
define!(WS_TILEDWINDOW, WS_OVERLAPPEDWINDOW);

define!(
  WS_OVERLAPPEDWINDOW,
  (WS_OVERLAPPED
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX)
);

define!(WS_POPUPWINDOW, (WS_POPUP | WS_BORDER | WS_SYSMENU));

define!(WS_CHILDWINDOW, (WS_CHILD));

define!(WS_EX_DLGMODALFRAME, 0x00000001);
define!(WS_EX_NOPARENTNOTIFY, 0x00000004);
define!(WS_EX_TOPMOST, 0x00000008);
define!(WS_EX_ACCEPTFILES, 0x00000010);
define!(WS_EX_TRANSPARENT, 0x00000020);
define!(WS_EX_MDICHILD, 0x00000040);
define!(WS_EX_TOOLWINDOW, 0x00000080);
define!(WS_EX_WINDOWEDGE, 0x00000100);
define!(WS_EX_CLIENTEDGE, 0x00000200);
define!(WS_EX_CONTEXTHELP, 0x00000400);
define!(WS_EX_RIGHT, 0x00001000);
define!(WS_EX_LEFT, 0x00000000);
define!(WS_EX_RTLREADING, 0x00002000);
define!(WS_EX_LTRREADING, 0x00000000);
define!(WS_EX_LEFTSCROLLBAR, 0x00004000);
define!(WS_EX_RIGHTSCROLLBAR, 0x00000000);
define!(WS_EX_CONTROLPARENT, 0x00010000);
define!(WS_EX_STATICEDGE, 0x00020000);
define!(WS_EX_APPWINDOW, 0x00040000);

define!(WS_EX_OVERLAPPEDWINDOW, (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE));
define!(
  WS_EX_PALETTEWINDOW,
  (WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST)
);
define!(WS_EX_LAYERED, 0x00080000);
define!(WS_EX_NOINHERITLAYOUT, 0x00100000);

define!(WS_EX_NOREDIRECTIONBITMAP, 0x00200000);
define!(WS_EX_LAYOUTRTL, 0x00400000);
define!(WS_EX_COMPOSITED, 0x02000000);
define!(WS_EX_NOACTIVATE, 0x08000000);

define!(WM_NULL, 0x0000);
define!(WM_CREATE, 0x0001);
define!(WM_DESTROY, 0x0002);
define!(WM_MOVE, 0x0003);
define!(WM_SIZE, 0x0005);
define!(WM_ACTIVATE, 0x0006);
define!(WM_SETFOCUS, 0x0007);
define!(WM_KILLFOCUS, 0x0008);
define!(WM_ENABLE, 0x000A);
define!(WM_SETREDRAW, 0x000B);
define!(WM_SETTEXT, 0x000C);
define!(WM_GETTEXT, 0x000D);
define!(WM_GETTEXTLENGTH, 0x000E);
define!(WM_PAINT, 0x000F);
define!(WM_CLOSE, 0x0010);
define!(WM_QUERYENDSESSION, 0x0011);
define!(WM_QUERYOPEN, 0x0013);
define!(WM_ENDSESSION, 0x0016);
define!(WM_QUIT, 0x0012);
define!(WM_ERASEBKGND, 0x0014);
define!(WM_SYSCOLORCHANGE, 0x0015);
define!(WM_SHOWWINDOW, 0x0018);
define!(WM_WININICHANGE, 0x001A);
define!(WM_SETTINGCHANGE, WM_WININICHANGE);
define!(WM_DEVMODECHANGE, 0x001B);
define!(WM_ACTIVATEAPP, 0x001C);
define!(WM_FONTCHANGE, 0x001D);
define!(WM_TIMECHANGE, 0x001E);
define!(WM_CANCELMODE, 0x001F);
define!(WM_SETCURSOR, 0x0020);
define!(WM_MOUSEACTIVATE, 0x0021);
define!(WM_CHILDACTIVATE, 0x0022);
define!(WM_QUEUESYNC, 0x0023);
define!(WM_GETMINMAXINFO, 0x0024);
define!(WM_PAINTICON, 0x0026);
define!(WM_ICONERASEBKGND, 0x0027);
define!(WM_NEXTDLGCTL, 0x0028);
define!(WM_SPOOLERSTATUS, 0x002A);
define!(WM_DRAWITEM, 0x002B);
define!(WM_MEASUREITEM, 0x002C);
define!(WM_DELETEITEM, 0x002D);
define!(WM_VKEYTOITEM, 0x002E);
define!(WM_CHARTOITEM, 0x002F);
define!(WM_SETFONT, 0x0030);
define!(WM_GETFONT, 0x0031);
define!(WM_SETHOTKEY, 0x0032);
define!(WM_GETHOTKEY, 0x0033);
define!(WM_QUERYDRAGICON, 0x0037);
define!(WM_COMPAREITEM, 0x0039);
define!(WM_GETOBJECT, 0x003D);
define!(WM_COMPACTING, 0x0041);
define!(WM_WINDOWPOSCHANGING, 0x0046);
define!(WM_WINDOWPOSCHANGED, 0x0047);
define!(WM_POWER, 0x0048);
define!(WM_COPYDATA, 0x004A);
define!(WM_CANCELJOURNAL, 0x004B);
define!(WM_NOTIFY, 0x004E);
define!(WM_INPUTLANGCHANGEREQUEST, 0x0050);
define!(WM_INPUTLANGCHANGE, 0x0051);
define!(WM_TCARD, 0x0052);
define!(WM_HELP, 0x0053);
define!(WM_USERCHANGED, 0x0054);
define!(WM_NOTIFYFORMAT, 0x0055);
define!(WM_CONTEXTMENU, 0x007B);
define!(WM_STYLECHANGING, 0x007C);
define!(WM_STYLECHANGED, 0x007D);
define!(WM_DISPLAYCHANGE, 0x007E);
define!(WM_GETICON, 0x007F);
define!(WM_SETICON, 0x0080);
define!(WM_NCCREATE, 0x0081);
define!(WM_NCDESTROY, 0x0082);
define!(WM_NCCALCSIZE, 0x0083);
define!(WM_NCHITTEST, 0x0084);
define!(WM_NCPAINT, 0x0085);
define!(WM_NCACTIVATE, 0x0086);
define!(WM_GETDLGCODE, 0x0087);
define!(WM_SYNCPAINT, 0x0088);
define!(WM_NCMOUSEMOVE, 0x00A0);
define!(WM_NCLBUTTONDOWN, 0x00A1);
define!(WM_NCLBUTTONUP, 0x00A2);
define!(WM_NCLBUTTONDBLCLK, 0x00A3);
define!(WM_NCRBUTTONDOWN, 0x00A4);
define!(WM_NCRBUTTONUP, 0x00A5);
define!(WM_NCRBUTTONDBLCLK, 0x00A6);
define!(WM_NCMBUTTONDOWN, 0x00A7);
define!(WM_NCMBUTTONUP, 0x00A8);
define!(WM_NCMBUTTONDBLCLK, 0x00A9);
define!(WM_NCXBUTTONDOWN, 0x00AB);
define!(WM_NCXBUTTONUP, 0x00AC);
define!(WM_NCXBUTTONDBLCLK, 0x00AD);
define!(WM_INPUT_DEVICE_CHANGE, 0x00FE);
define!(WM_INPUT, 0x00FF);
define!(WM_KEYFIRST, 0x0100);
define!(WM_KEYDOWN, 0x0100);
define!(WM_KEYUP, 0x0101);
define!(WM_CHAR, 0x0102);
define!(WM_DEADCHAR, 0x0103);
define!(WM_SYSKEYDOWN, 0x0104);
define!(WM_SYSKEYUP, 0x0105);
define!(WM_SYSCHAR, 0x0106);
define!(WM_SYSDEADCHAR, 0x0107);
define!(WM_UNICHAR, 0x0109);
define!(WM_KEYLAST, 0x0109);
define!(WM_IME_STARTCOMPOSITION, 0x010D);
define!(WM_IME_ENDCOMPOSITION, 0x010E);
define!(WM_IME_COMPOSITION, 0x010F);
define!(WM_IME_KEYLAST, 0x010F);
define!(WM_INITDIALOG, 0x0110);
define!(WM_COMMAND, 0x0111);
define!(WM_SYSCOMMAND, 0x0112);
define!(WM_TIMER, 0x0113);
define!(WM_HSCROLL, 0x0114);
define!(WM_VSCROLL, 0x0115);
define!(WM_INITMENU, 0x0116);
define!(WM_INITMENUPOPUP, 0x0117);
define!(WM_GESTURE, 0x0119);
define!(WM_GESTURENOTIFY, 0x011A);
define!(WM_MENUSELECT, 0x011F);
define!(WM_MENUCHAR, 0x0120);
define!(WM_ENTERIDLE, 0x0121);
define!(WM_MENURBUTTONUP, 0x0122);
define!(WM_MENUDRAG, 0x0123);
define!(WM_MENUGETOBJECT, 0x0124);
define!(WM_UNINITMENUPOPUP, 0x0125);
define!(WM_MENUCOMMAND, 0x0126);
define!(WM_CHANGEUISTATE, 0x0127);
define!(WM_UPDATEUISTATE, 0x0128);
define!(WM_QUERYUISTATE, 0x0129);
define!(WM_CTLCOLORMSGBOX, 0x0132);
define!(WM_CTLCOLOREDIT, 0x0133);
define!(WM_CTLCOLORLISTBOX, 0x0134);
define!(WM_CTLCOLORBTN, 0x0135);
define!(WM_CTLCOLORDLG, 0x0136);
define!(WM_CTLCOLORSCROLLBAR, 0x0137);
define!(WM_CTLCOLORSTATIC, 0x0138);
define!(WM_MOUSEFIRST, 0x0200);
define!(WM_MOUSEMOVE, 0x0200);
define!(WM_LBUTTONDOWN, 0x0201);
define!(WM_LBUTTONUP, 0x0202);
define!(WM_LBUTTONDBLCLK, 0x0203);
define!(WM_RBUTTONDOWN, 0x0204);
define!(WM_RBUTTONUP, 0x0205);
define!(WM_RBUTTONDBLCLK, 0x0206);
define!(WM_MBUTTONDOWN, 0x0207);
define!(WM_MBUTTONUP, 0x0208);
define!(WM_MBUTTONDBLCLK, 0x0209);
define!(WM_MOUSEWHEEL, 0x020A);
define!(WM_XBUTTONDOWN, 0x020B);
define!(WM_XBUTTONUP, 0x020C);
define!(WM_XBUTTONDBLCLK, 0x020D);
define!(WM_MOUSEHWHEEL, 0x020E);
define!(WM_MOUSELAST, 0x020E);
define!(WM_PARENTNOTIFY, 0x0210);
define!(WM_ENTERMENULOOP, 0x0211);
define!(WM_EXITMENULOOP, 0x0212);
define!(WM_NEXTMENU, 0x0213);
define!(WM_SIZING, 0x0214);
define!(WM_CAPTURECHANGED, 0x0215);
define!(WM_MOVING, 0x0216);
define!(WM_POWERBROADCAST, 0x0218);
define!(WM_DEVICECHANGE, 0x0219);
define!(WM_MDICREATE, 0x0220);
define!(WM_MDIDESTROY, 0x0221);
define!(WM_MDIACTIVATE, 0x0222);
define!(WM_MDIRESTORE, 0x0223);
define!(WM_MDINEXT, 0x0224);
define!(WM_MDIMAXIMIZE, 0x0225);
define!(WM_MDITILE, 0x0226);
define!(WM_MDICASCADE, 0x0227);
define!(WM_MDIICONARRANGE, 0x0228);
define!(WM_MDIGETACTIVE, 0x0229);
define!(WM_MDISETMENU, 0x0230);
define!(WM_ENTERSIZEMOVE, 0x0231);
define!(WM_EXITSIZEMOVE, 0x0232);
define!(WM_DROPFILES, 0x0233);
define!(WM_MDIREFRESHMENU, 0x0234);
define!(WM_POINTERDEVICECHANGE, 0x238);
define!(WM_POINTERDEVICEINRANGE, 0x239);
define!(WM_POINTERDEVICEOUTOFRANGE, 0x23A);
define!(WM_TOUCH, 0x0240);
define!(WM_NCPOINTERUPDATE, 0x0241);
define!(WM_NCPOINTERDOWN, 0x0242);
define!(WM_NCPOINTERUP, 0x0243);
define!(WM_POINTERUPDATE, 0x0245);
define!(WM_POINTERDOWN, 0x0246);
define!(WM_POINTERUP, 0x0247);
define!(WM_POINTERENTER, 0x0249);
define!(WM_POINTERLEAVE, 0x024A);
define!(WM_POINTERACTIVATE, 0x024B);
define!(WM_POINTERCAPTURECHANGED, 0x024C);
define!(WM_TOUCHHITTESTING, 0x024D);
define!(WM_POINTERWHEEL, 0x024E);
define!(WM_POINTERHWHEEL, 0x024F);
define!(WM_POINTERROUTEDTO, 0x0251);
define!(WM_POINTERROUTEDAWAY, 0x0252);
define!(WM_POINTERROUTEDRELEASED, 0x0253);
define!(WM_IME_SETCONTEXT, 0x0281);
define!(WM_IME_NOTIFY, 0x0282);
define!(WM_IME_CONTROL, 0x0283);
define!(WM_IME_COMPOSITIONFULL, 0x0284);
define!(WM_IME_SELECT, 0x0285);
define!(WM_IME_CHAR, 0x0286);
define!(WM_IME_REQUEST, 0x0288);
define!(WM_IME_KEYDOWN, 0x0290);
define!(WM_IME_KEYUP, 0x0291);
define!(WM_MOUSEHOVER, 0x02A1);
define!(WM_MOUSELEAVE, 0x02A3);
define!(WM_NCMOUSEHOVER, 0x02A0);
define!(WM_NCMOUSELEAVE, 0x02A2);
define!(WM_WTSSESSION_CHANGE, 0x02B1);
define!(WM_TABLET_FIRST, 0x02c0);
define!(WM_TABLET_LAST, 0x02df);
define!(WM_DPICHANGED, 0x02E0);
define!(WM_DPICHANGED_BEFOREPARENT, 0x02E2);
define!(WM_DPICHANGED_AFTERPARENT, 0x02E3);
define!(WM_GETDPISCALEDSIZE, 0x02E4);
define!(WM_CUT, 0x0300);
define!(WM_COPY, 0x0301);
define!(WM_PASTE, 0x0302);
define!(WM_CLEAR, 0x0303);
define!(WM_UNDO, 0x0304);
define!(WM_RENDERFORMAT, 0x0305);
define!(WM_RENDERALLFORMATS, 0x0306);
define!(WM_DESTROYCLIPBOARD, 0x0307);
define!(WM_DRAWCLIPBOARD, 0x0308);
define!(WM_PAINTCLIPBOARD, 0x0309);
define!(WM_VSCROLLCLIPBOARD, 0x030A);
define!(WM_SIZECLIPBOARD, 0x030B);
define!(WM_ASKCBFORMATNAME, 0x030C);
define!(WM_CHANGECBCHAIN, 0x030D);
define!(WM_HSCROLLCLIPBOARD, 0x030E);
define!(WM_QUERYNEWPALETTE, 0x030F);
define!(WM_PALETTEISCHANGING, 0x0310);
define!(WM_PALETTECHANGED, 0x0311);
define!(WM_HOTKEY, 0x0312);
define!(WM_PRINT, 0x0317);
define!(WM_PRINTCLIENT, 0x0318);
define!(WM_APPCOMMAND, 0x0319);
define!(WM_THEMECHANGED, 0x031A);
define!(WM_CLIPBOARDUPDATE, 0x031D);
define!(WM_DWMCOMPOSITIONCHANGED, 0x031E);
define!(WM_DWMNCRENDERINGCHANGED, 0x031F);
define!(WM_DWMCOLORIZATIONCOLORCHANGED, 0x0320);
define!(WM_DWMWINDOWMAXIMIZEDCHANGE, 0x0321);
define!(WM_DWMSENDICONICTHUMBNAIL, 0x0323);
define!(WM_DWMSENDICONICLIVEPREVIEWBITMAP, 0x0326);
define!(WM_GETTITLEBARINFOEX, 0x033F);
define!(WM_HANDHELDFIRST, 0x0358);
define!(WM_HANDHELDLAST, 0x035F);
define!(WM_AFXFIRST, 0x0360);
define!(WM_AFXLAST, 0x037F);
define!(WM_PENWINFIRST, 0x0380);
define!(WM_PENWINLAST, 0x038F);
define!(WM_APP, 0x8000);
define!(WM_USER, 0x0400);

define!(VK_LBUTTON, 0x01);
define!(VK_RBUTTON, 0x02);
define!(VK_CANCEL, 0x03);
define!(VK_BACK, 0x08);
define!(VK_TAB, 0x09);
define!(VK_CLEAR, 0x0C);
define!(VK_RETURN, 0x0D);
define!(VK_SHIFT, 0x10);
define!(VK_CONTROL, 0x11);
define!(VK_MENU, 0x12);
define!(VK_PAUSE, 0x13);
define!(VK_CAPITAL, 0x14);
define!(VK_KANA, 0x15);
define!(VK_HANGUL, 0x15);
define!(VK_JUNJA, 0x17);
define!(VK_FINAL, 0x18);
define!(VK_HANJA, 0x19);
define!(VK_KANJI, 0x19);
define!(VK_ESCAPE, 0x1B);
define!(VK_CONVERT, 0x1C);
define!(VK_NONCONVERT, 0x1D);
define!(VK_ACCEPT, 0x1E);
define!(VK_MODECHANGE, 0x1F);
define!(VK_SPACE, 0x20);
define!(VK_PRIOR, 0x21);
define!(VK_NEXT, 0x22);
define!(VK_END, 0x23);
define!(VK_HOME, 0x24);
define!(VK_LEFT, 0x25);
define!(VK_UP, 0x26);
define!(VK_RIGHT, 0x27);
define!(VK_DOWN, 0x28);
define!(VK_SELECT, 0x29);
define!(VK_PRINT, 0x2A);
define!(VK_EXECUTE, 0x2B);
define!(VK_SNAPSHOT, 0x2C);
define!(VK_INSERT, 0x2D);
define!(VK_DELETE, 0x2E);
define!(VK_HELP, 0x2F);
define!(VK_LWIN, 0x5B);
define!(VK_RWIN, 0x5C);
define!(VK_APPS, 0x5D);
define!(VK_SLEEP, 0x5F);
define!(VK_NUMPAD0, 0x60);
define!(VK_NUMPAD1, 0x61);
define!(VK_NUMPAD2, 0x62);
define!(VK_NUMPAD3, 0x63);
define!(VK_NUMPAD4, 0x64);
define!(VK_NUMPAD5, 0x65);
define!(VK_NUMPAD6, 0x66);
define!(VK_NUMPAD7, 0x67);
define!(VK_NUMPAD8, 0x68);
define!(VK_NUMPAD9, 0x69);
define!(VK_MULTIPLY, 0x6A);
define!(VK_ADD, 0x6B);
define!(VK_SEPARATOR, 0x6C);
define!(VK_SUBTRACT, 0x6D);
define!(VK_DECIMAL, 0x6E);
define!(VK_DIVIDE, 0x6F);
define!(VK_F1, 0x70);
define!(VK_F2, 0x71);
define!(VK_F3, 0x72);
define!(VK_F4, 0x73);
define!(VK_F5, 0x74);
define!(VK_F6, 0x75);
define!(VK_F7, 0x76);
define!(VK_F8, 0x77);
define!(VK_F9, 0x78);
define!(VK_F10, 0x79);
define!(VK_F11, 0x7A);
define!(VK_F12, 0x7B);
define!(VK_F13, 0x7C);
define!(VK_F14, 0x7D);
define!(VK_F15, 0x7E);
define!(VK_F16, 0x7F);
define!(VK_F17, 0x80);
define!(VK_F18, 0x81);
define!(VK_F19, 0x82);
define!(VK_F20, 0x83);
define!(VK_F21, 0x84);
define!(VK_F22, 0x85);
define!(VK_F23, 0x86);
define!(VK_F24, 0x87);
define!(VK_NUMLOCK, 0x90);
define!(VK_SCROLL, 0x91);
define!(VK_LSHIFT, 0xA0);
define!(VK_RSHIFT, 0xA1);
define!(VK_LCONTROL, 0xA2);
define!(VK_RCONTROL, 0xA3);
define!(VK_LMENU, 0xA4);
define!(VK_RMENU, 0xA5);
define!(VK_BROWSER_BACK, 0xA6);
define!(VK_BROWSER_FORWARD, 0xA7);
define!(VK_BROWSER_REFRESH, 0xA8);
define!(VK_BROWSER_STOP, 0xA9);
define!(VK_BROWSER_SEARCH, 0xAA);
define!(VK_BROWSER_FAVORITES, 0xAB);
define!(VK_BROWSER_HOME, 0xAC);
define!(VK_VOLUME_MUTE, 0xAD);
define!(VK_VOLUME_DOWN, 0xAE);
define!(VK_VOLUME_UP, 0xAF);
define!(VK_MEDIA_NEXT_TRACK, 0xB0);
define!(VK_MEDIA_PREV_TRACK, 0xB1);
define!(VK_MEDIA_STOP, 0xB2);
define!(VK_MEDIA_PLAY_PAUSE, 0xB3);
define!(VK_LAUNCH_MAIL, 0xB4);
define!(VK_LAUNCH_MEDIA_SELECT, 0xB5);
define!(VK_LAUNCH_APP1, 0xB6);
define!(VK_LAUNCH_APP2, 0xB7);
define!(VK_OEM_8, 0xDF);
define!(VK_PROCESSKEY, 0xE5);
define!(VK_ICO_CLEAR, 0xE6);
define!(VK_PACKET, 0xE7);
define!(VK_OEM_RESET, 0xE9);
define!(VK_OEM_JUMP, 0xEA);
define!(VK_OEM_PA1, 0xEB);
define!(VK_OEM_PA2, 0xEC);
define!(VK_OEM_PA3, 0xED);
define!(VK_OEM_WSCTRL, 0xEE);
define!(VK_OEM_CUSEL, 0xEF);
define!(VK_OEM_ATTN, 0xF0);
define!(VK_OEM_FINISH, 0xF1);
define!(VK_OEM_COPY, 0xF2);
define!(VK_OEM_AUTO, 0xF3);
define!(VK_OEM_ENLW, 0xF4);
define!(VK_OEM_BACKTAB, 0xF5);
define!(VK_ATTN, 0xF6);
define!(VK_CRSEL, 0xF7);
define!(VK_EXSEL, 0xF8);
define!(VK_EREOF, 0xF9);
define!(VK_PLAY, 0xFA);
define!(VK_ZOOM, 0xFB);
define!(VK_NONAME, 0xFC);
define!(VK_PA1, 0xFD);
define!(VK_OEM_CLEAR, 0xFE);

#[allow(unused_macros)]
macro_rules! HIWORD {
  ($l:expr) => {
    ((($l as DWORD_PTR) >> 16) & 0xFFFF) as WORD
  };
}

#[allow(unused_macros)]
macro_rules! LOWORD {
  ($l:expr) => {
    (($l as DWORD_PTR) & 0xFFFF) as WORD
  };
}

#[allow(unused_macros)]
macro_rules! GET_X_LPARAM {
  ($lp:expr) => {
    LOWORD!($lp) as c_short as c_int
  };
}

#[allow(unused_macros)]
macro_rules! GET_Y_LPARAM {
  ($lp:expr) => {
    HIWORD!($lp) as c_short as c_int
  };
}

macro_rules! c_struct {
  ($(#[$m:meta])* $name:ident { $($field:ident : $field_type:ty),* $(,)? }) => {
    $(#[$m])*
    #[repr(C)]
    #[cfg_attr(feature = "struct_debug", derive(Debug))]
    #[cfg_attr(feature = "struct_eq", derive(PartialEq, Eq))]
    #[cfg_attr(feature = "struct_hash", derive(Hash))]
    pub struct $name {
      $($field : $field_type),*
    }
    impl Copy for $name {}
    impl Clone for $name { fn clone(&self) -> Self { *self } }
    #[cfg(feature = "struct_default")]
    impl Default for $name {
      fn default() -> Self {
        unsafe { core::mem::zeroed() }
      }
    }
  };
}

c_struct! { RECT {
  left: LONG,
  top: LONG,
  right: LONG,
  bottom: LONG,
}}

c_struct! { POINT {
  x: LONG,
  y: LONG,
}}

c_struct! { PIXELFORMATDESCRIPTOR {
  nSize: WORD,
  nVersion: WORD,
  dwFlags: DWORD,
  iPixelType: BYTE,
  cColorBits: BYTE,
  cRedBits: BYTE,
  cRedShift: BYTE,
  cGreenBits: BYTE,
  cGreenShift: BYTE,
  cBlueBits: BYTE,
  cBlueShift: BYTE,
  cAlphaBits: BYTE,
  cAlphaShift: BYTE,
  cAccumBits: BYTE,
  cAccumRedBits: BYTE,
  cAccumGreenBits: BYTE,
  cAccumBlueBits: BYTE,
  cAccumAlphaBits: BYTE,
  cDepthBits: BYTE,
  cStencilBits: BYTE,
  cAuxBuffers: BYTE,
  iLayerType: BYTE,
  bReserved: BYTE,
  dwLayerMask: DWORD,
  dwVisibleMask: DWORD,
  dwDamageMask: DWORD,
}}

c_struct! { MSG {
  hwnd: HWND,
  message: UINT,
  wParam: WPARAM,
  lParam: LPARAM,
  time: DWORD,
  pt: POINT,
  lPrivate: DWORD,
}}

c_struct! { WNDCLASSW {
  style: UINT,
  lpfnWndProc: WNDPROC,
  cbClsExtra: c_int,
  cbWndExtra: c_int,
  hInstance: HINSTANCE,
  hIcon: HICON,
  hCursor: HCURSOR,
  hbrBackground: HBRUSH,
  lpszMenuName: LPCWSTR,
  lpszClassName: LPCWSTR,
}}

#[link(name = "Kernel32")]
extern "system" {
  /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

  /// [`LoadLibraryA`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya)
  pub fn LoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE;

  /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
  pub fn GetLastError() -> DWORD;
}

#[link(name = "Opengl32")]
extern "system" {
  /// [`wglCreateContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglcreatecontext)
  pub fn wglCreateContext(Arg1: HDC) -> HGLRC;

  /// [`wglDeleteContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wgldeletecontext)
  pub fn wglDeleteContext(Arg1: HGLRC) -> BOOL;

  /// [`wglGetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
  pub fn wglGetProcAddress(Arg1: LPCSTR) -> PROC;

  /// [`wglMakeCurrent`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglmakecurrent)
  pub fn wglMakeCurrent(arg1: HDC, arg2: HGLRC) -> BOOL;
}

#[link(name = "Gdi32")]
extern "system" {
  /// [`ChoosePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
  pub fn ChoosePixelFormat(
    hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR,
  ) -> c_int;

  /// [`SetPixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
  pub fn SetPixelFormat(
    hdc: HDC, format: c_int, ppfd: *const PIXELFORMATDESCRIPTOR,
  ) -> BOOL;

  /// [`SwapBuffers`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-swapbuffers)
  pub fn SwapBuffers(arg1: HDC) -> BOOL;
}

#[link(name = "User32")]
extern "system" {
  /// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
  pub fn AdjustWindowRectEx(
    lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD,
  ) -> BOOL;

  /// [`CloseWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
  pub fn CloseWindow(hWnd: HWND) -> BOOL;

  /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
  pub fn CreateWindowExW(
    dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
    dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
    hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,
  ) -> HWND;

  /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
  pub fn DefWindowProcW(
    hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
  ) -> LRESULT;

  /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
  pub fn DestroyWindow(hWnd: HWND) -> BOOL;

  /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
  pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

  /// [`GetClientRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
  pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;

  /// [`GetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
  pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;

  /// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
  pub fn GetDC(hWnd: HWND) -> HDC;

  /// [`GetMessageTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagetime)
  pub fn GetMessageTime() -> LONG;

  /// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
  pub fn GetSystemMetrics(nIndex: c_int) -> c_int;

  /// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
  pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;

  /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
  pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

  /// [`MoveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
  pub fn MoveWindow(
    hWnd: HWND, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
    bRepaint: BOOL,
  ) -> BOOL;

  /// [`PeekMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
  pub fn PeekMessageW(
    lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
    wRemoveMsg: UINT,
  ) -> BOOL;

  /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
  pub fn PostQuitMessage(nExitCode: c_int);

  /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
  pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

  /// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
  pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;

  /// [`SetCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursor)
  pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;

  /// [`SetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
  pub fn SetCursorPos(x: c_int, y: c_int) -> BOOL;

  /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
  pub fn SetWindowLongPtrW(
    hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR,
  ) -> LONG_PTR;

  /// [`SetWindowTextW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
  pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;

  /// [`ShowCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
  pub fn ShowCursor(bShow: BOOL) -> c_int;

  /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
  pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

  /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
  pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
}
