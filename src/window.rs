use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea,
    Win32::Graphics::Gdi::ValidateRect,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::Controls::{GetThemeSysSize, MARGINS},
    Win32::UI::WindowsAndMessaging::*,
};

use crate::utils::{rgb, str_to_pcwstr, GET_X_LPARAM, GET_Y_LPARAM};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};
use std::ffi::c_void;
use std::sync::Once;
use windows::Win32::Foundation::LRESULT;

/// Default background colour
const BGCOLOUR: u32 = rgb(52, 55, 60);

/// Required until I figure out how to handle multiple windows
static REGISTER_WINDOW_CLASS: Once = Once::new();

#[derive(Debug)]
#[repr(C)]
pub struct Window {
    instance: HINSTANCE,
    handle: HWND,
}

impl Window {
    pub fn test_correct(&self){
        println!("I'm a little window, short and stdout. \
        This is my handle: {:?} this is my instance {:?}",
                 &self.handle, &self.instance)
    }

    pub fn new(title: &str, window_class_name: &str) -> Result<Box<Self>> {
        let hinstance = unsafe { GetModuleHandleW(None).ok() }?;

        REGISTER_WINDOW_CLASS.call_once(|| {
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                hInstance: hinstance, // A handle to the process that contains the window procedure
                style: CS_HREDRAW | CS_VREDRAW, // Styling
                hIcon: unsafe { LoadIconW(None, IDI_EXCLAMATION) },
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() }, // A handle to the class cursor
                hbrBackground: unsafe { CreateSolidBrush(BGCOLOUR).ok().unwrap() },
                lpszClassName: str_to_pcwstr(window_class_name),
                lpfnWndProc: Some(Self::wnd_proc_sys), // A pointer to the window procedure - defined below
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassExW(&wc) }, 0);
        });

        let mut window = Box::new(Self {
            instance: hinstance,
            handle: HWND(0),
        });

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                window_class_name,
                title,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                // mutable reference to raw pointer where we let the compiler work out the type '_'
                // we then cast this raw pointer to *mut c_void type `_` as is required
                // &mut window as *mut _ as *mut c_void,
                window.as_mut() as *mut _ as _,
            )
            .ok()?
        };

        if hwnd.is_invalid() {
            return Err(unsafe {
                Error::new(GetLastError().into(), "Failed to create window".into())
            });
        }

        Ok(window)
    }

    fn wnd_proc(
        &mut self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<LRESULT> {
        match message {
            WM_CREATE => unsafe {
                let margins = MARGINS {
                    cxLeftWidth: 1,
                    cxRightWidth: 1,
                    cyTopHeight: 1,
                    cyBottomHeight: 1,
                };
                DwmExtendFrameIntoClientArea(self.handle, &margins).unwrap();
            }
            WM_PAINT => unsafe {
                ValidateRect(self.handle, std::ptr::null());
            }
            WM_NCCALCSIZE => {
                // Stop this msg passing to the default procedure as it screws up borderless
                return Some(LRESULT(0))
            }
            // Non-client hit test
            WM_NCHITTEST => {
                return Some(
                    Region::hit_test(
                        self.handle,
                        POINT {
                            x: GET_X_LPARAM(lparam.0 as u32),
                            y: GET_Y_LPARAM(lparam.0 as u32),
                        },
                ))
            }
            _ => {}
        }
        None
    }

    // The external window system procedure.
    // Handles the window pointer and passes on the message to the wnd_proc method
    unsafe extern "system" fn wnd_proc_sys(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // If the message is a creation message then set the pointer,
        // otherwise get the pointer and pass along

        let l_result  = if message == WM_NCCREATE {
            // Set Window pointer
            let cs = lparam.0 as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;
            (*this).handle = hwnd;
            if hwnd.is_invalid() {
                panic!("Cannot recover: Window handle is invalid");
            }
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, this as _);
            None
        }
        else {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;
            if let Some(this) = this.as_mut(){
                this.wnd_proc(message, wparam, lparam)
            } else {
                None
            }
        };

        if let Some(l) = l_result {
            return l
        }

        DefWindowProcW(hwnd, message, wparam, lparam)
    }

    pub fn start(&self) {
        let mut message = MSG::default();
        unsafe {
            while GetMessageW(&mut message, self.handle, 0, 0).into() {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut hdl = Win32Handle::empty();
        hdl.hinstance = self.instance.0 as *mut c_void;
        hdl.hwnd = self.handle.0 as *mut c_void;

        RawWindowHandle::Win32(hdl)
    }
}

pub(crate) fn get_titlebar_height() -> i32 {
    unsafe {
        GetThemeSysSize(0, SM_CYSIZE.0 as i32) + GetThemeSysSize(0, SM_CXPADDEDBORDER.0 as i32) * 2
    }
}

/// Gets the window area for the purposes of extending
/// the client area into the window area and thus removing the titlebar
fn get_window_rect(hwnd: HWND) -> Result<RECT> {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe { GetWindowRect(hwnd, &mut rect).ok()? };

    Ok(rect)
}

/// an enum defining the four corners of a window
#[rustfmt::skip]
pub(crate) enum Region {
    Left    = 0b0001,
    Top     = 0b0010,
    Right   = 0b0100,
    Bottom  = 0b1000,
}

impl Region {
    /// A hit check for the location of the cursor and unification of the values
    pub(crate) fn hit_test(hwnd: HWND, cursor: POINT) -> LRESULT {
        let border = unsafe {
            POINT {
                x: GetSystemMetrics(SM_CXFRAME) + GetSystemMetrics(SM_CXPADDEDBORDER),
                y: GetSystemMetrics(SM_CYFRAME) + GetSystemMetrics(SM_CXPADDEDBORDER),
            }
        };

        let rect = get_window_rect(hwnd).unwrap();

        let result = {
            (Self::Left as u8 * (cursor.x < (rect.left + border.x)) as u8)
                | (Self::Top as u8 * (cursor.y < (rect.top + border.y)) as u8)
                | (Self::Right as u8 * (cursor.x >= (rect.right - border.x)) as u8)
                | (Self::Bottom as u8 * (cursor.y >= (rect.bottom - border.y)) as u8)
        };

        let hit = match result {
            0b0000 => {
                if cursor.y < rect.top + get_titlebar_height() {
                    HTCAPTION // TODO: This is a temporary solution to titlebar
                } else {
                    HTCLIENT
                }
            }
            0b0001 => HTLEFT,
            0b0010 => HTTOP,
            0b0100 => HTRIGHT,
            0b1000 => HTBOTTOM,
            0b0011 => HTTOPLEFT,
            0b0110 => HTTOPRIGHT,
            0b1001 => HTBOTTOMLEFT,
            0b1100 => HTBOTTOMRIGHT,
            _ => HTNOWHERE,
        };

        LRESULT(hit as isize)
    }
}
