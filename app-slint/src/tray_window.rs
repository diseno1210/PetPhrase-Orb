//! Keep every PetPhrase window out of the taskbar and Alt-Tab.
//! ITaskbarList::DeleteTab alone is temporary: Explorer can restore it on show.

use slint::winit_030::{winit, WinitWindowAccessor};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

#[link(name = "user32")]
extern "system" {
    fn GetWindowLongPtrW(hwnd: isize, index: i32) -> isize;
    fn SetWindowLongPtrW(hwnd: isize, index: i32, value: isize) -> isize;
    fn SetWindowPos(hwnd: isize, after: isize, x: i32, y: i32, cx: i32, cy: i32, flags: u32)
        -> i32;
}

pub fn configure(window: &slint::Window) {
    window.with_winit_window(|w: &winit::window::Window| {
        use winit::platform::windows::WindowExtWindows;
        let Ok(handle) = w.window_handle() else {
            return;
        };
        let RawWindowHandle::Win32(handle) = handle.as_raw() else {
            return;
        };
        let hwnd = handle.hwnd.get();
        const GWL_EXSTYLE: i32 = -20;
        const WS_EX_APPWINDOW: isize = 0x0004_0000;
        const WS_EX_TOOLWINDOW: isize = 0x0000_0080;
        // SAFETY: live HWND borrowed from winit, on the window's event-loop thread.
        // Only task-switcher styles change; focus, input, transparency and z-order stay intact.
        unsafe {
            let old = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let style = (old & !WS_EX_APPWINDOW) | WS_EX_TOOLWINDOW;
            if old != style {
                w.set_skip_taskbar(true);
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style);
                // NOSIZE | NOMOVE | NOZORDER | NOACTIVATE | FRAMECHANGED
                SetWindowPos(hwnd, 0, 0, 0, 0, 0, 0x0037);
            }
            debug_assert_eq!(GetWindowLongPtrW(hwnd, GWL_EXSTYLE) & WS_EX_APPWINDOW, 0);
            debug_assert_ne!(GetWindowLongPtrW(hwnd, GWL_EXSTYLE) & WS_EX_TOOLWINDOW, 0);
        }
    });
}
