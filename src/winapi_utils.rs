#![allow(dead_code)]
//! This file is a QA on unsafe code from the windows API, with commentary according to the MSDN
//! documentation and windows-rs docs.

use std::ffi::c_void;
use windows::core::*;

use windows::Win32::{
    Foundation::*,
    Graphics::Dwm::DwmExtendFrameIntoClientArea,
    Graphics::Gdi::ValidateRect,
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::Controls::{GetThemeSysSize, MARGINS},
    UI::WindowsAndMessaging::*,
    // Foundation::LRESULT
};

const BORDERLESS_MARGINS: MARGINS = MARGINS {
    cxLeftWidth: 1,
    cxRightWidth: 1,
    cyTopHeight: 1,
    cyBottomHeight: 1,
};

const FLAT_MARGINS: MARGINS = MARGINS {
    cxLeftWidth: 0,
    cxRightWidth: 0,
    cyTopHeight: 0,
    cyBottomHeight: 0,
};

/// alias for GetModuleHandleW
/// **Do not use this handle outside of the current module.**
/// https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/LibraryLoader/fn.GetModuleHandleW.html
/// Retrieves a module handle for the current module.
pub fn get_current_module_handle() -> Result<HINSTANCE> {
    // In a multi-threaded application this could be dangerous and cause a race condition.
    // "If this parameter is NULL, GetModuleHandle returns a handle to the file used to create
    // the calling process (.exe file)."
    // So long as this handle is only used for the calling process it is safe.
    // Runtime check included for verification.
    let hinstance = unsafe { GetModuleHandleW(None).ok() }?;
    Ok(hinstance)
}

/// Gets a handle to the default application icon.
/// Supersceded by loadimage, but still useful in some circumstances
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw
#[inline]
pub fn default_app_icon() -> Result<HICON> {
    // simple wrapper to check if handle is null.
    // removes the possibility of incorrectly typing icon name
    let hicon = unsafe { LoadIconW(None, IDI_APPLICATION).ok() }?;
    Ok(hicon)
}

/// Gets a handle to the warning icon.
/// Supersceded by loadimage, but still useful in some circumstances
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw
#[inline]
pub fn warning_icon() -> Result<HICON> {
    // simple wrapper to check if handle is null.
    // removes the possibility of incorrectly typing icon name
    let hicon = unsafe { LoadIconW(None, IDI_EXCLAMATION).ok() }?;
    Ok(hicon)
}

/// Gets a handle to the arrow cursor.
/// Supersceded by loadimage, but still useful in some circumstances
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw
#[inline]
pub fn default_cursor() -> Result<HCURSOR> {
    // simple wrapper to check if handle is null.
    // removes the possibility of incorrectly typing icon name
    // Other default cursor functionality should be implemented by the use of an enum
    unsafe { LoadCursorW(None, IDC_ARROW).ok() }
}

/// creates a new solid brush given an rgb u32 input
pub fn solid_brush(rgb: u32) -> Result<HBRUSH> {
    let hbrush = unsafe { CreateSolidBrush(rgb).ok() }?;
    Ok(hbrush)
}

#[inline]
pub fn get_last_error(message: &str) -> Error {
    unsafe { Error::new(GetLastError().into(), message.into()) }
}

/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.RegisterClassExW.html
pub fn register_window_class(class: &WNDCLASSEXW) -> Result<u16> {
    let atom = unsafe { RegisterClassExW(class) };
    if atom != 0 {
        Ok(atom)
    } else {
        Err(get_last_error("Failed to register window class"))
    }
}

/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.CreateWindowExW.html
#[allow(clippy::too_many_arguments)]
pub fn create_window(
    dwexstyle: WINDOW_EX_STYLE,
    class_name: &str,
    title: &str,
    dwstyle: WINDOW_STYLE,
    position: [i32; 2],
    size: [i32; 2],
    hinstance: HINSTANCE,
    lpparam: *const c_void,
) -> Result<HWND> {
    if hinstance.is_invalid() {
        panic!("Supplied an invalid instance handle to create window");
    }

    let hwnd = unsafe {
        CreateWindowExW(
            dwexstyle,
            class_name,
            title,
            dwstyle,
            position[0],
            position[1],
            size[0],
            size[1],
            // Will consider this implementation in another function if child windows required
            None,
            // Will consider this implementation in another function if child windows required
            None,
            hinstance,
            lpparam,
        )
        .ok()?
    };

    if hwnd.is_invalid() {
        return Err(get_last_error("Failed to create window"));
    }

    Ok(hwnd)
}

/// Gets the window area for the purposes of extending
/// the client area into the window area and thus removing the titlebar
pub fn get_window_rect(hwnd: HWND) -> Result<RECT> {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe { GetWindowRect(hwnd, &mut rect).ok()? };

    Ok(rect)
}

#[inline]
pub fn get_titlebar_height() -> i32 {
    unsafe {
        GetThemeSysSize(0, SM_CYSIZE.0 as i32) + GetThemeSysSize(0, SM_CXPADDEDBORDER.0 as i32) * 2
    }
}

/// Function get get the system border width and height
#[inline]
pub fn get_border() -> POINT {
    unsafe {
        POINT {
            x: GetSystemMetrics(SM_CXFRAME) + GetSystemMetrics(SM_CXPADDEDBORDER),
            y: GetSystemMetrics(SM_CYFRAME) + GetSystemMetrics(SM_CXPADDEDBORDER),
        }
    }
}

/// General window style description
pub enum WindowStyle {
    /// No titlebar, border is overridden by backend
    Borderless,
    /// No titlebar, no drop shadow on window. Useful for splash screens and windows with transparency
    FlatBorderless,
}

/// Primarily used to create borderless windows
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Dwm/fn.DwmExtendFrameIntoClientArea.html
/// https://docs.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmextendframeintoclientarea
pub fn extend_frame_into_client_area(handle: HWND, style: &WindowStyle) -> Result<()> {
    let margins = match style {
        WindowStyle::FlatBorderless => FLAT_MARGINS,
        WindowStyle::Borderless => BORDERLESS_MARGINS,
    };

    if handle.is_invalid() {
        panic!("Cannot extend frame into client area due to invalid window handle")
    }

    unsafe { DwmExtendFrameIntoClientArea(handle, &margins) }
}

/// post a quit message with an exit code.
/// Simply Requests that the thread closes
/// Convention states to post 0 to indicate success.
/// Negative values are always an error.
/// Positive values are sometimes errors, sometimes information
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.PostQuitMessage.html
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage
#[inline]
pub fn post_quit_message(nexitcode: i32) {
    // MSDN lists no concerns so it's deemed safe
    unsafe { PostQuitMessage(nexitcode) };
}

/// Used to redraw the window
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect
/// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.ValidateRect.html
pub fn validate_rect(handle: HWND) -> Result<()> {
    // the second argument here is lprect: *const RECT which is:
    // A pointer to a RECT structure that contains the client coordinates of the rectangle to be
    // removed from the update region. If this parameter is NULL, the entire client area is removed.
    // We will just leave this as null for now so that the entire window is redrawn
    unsafe { ValidateRect(handle, std::ptr::null()).ok()? };
    Ok(())
}
