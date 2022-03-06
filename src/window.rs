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
use crate::render::State;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};
use std::ffi::c_void;
use std::sync::Once;
use windows::Win32::Foundation::LRESULT;

/// Default background colour
const BGCOLOUR: u32 = rgb(52, 55, 60);

/// Required until I figure out how to handle multiple windows
static REGISTER_WINDOW_CLASS: Once = Once::new();

pub enum Event {
    Resize,
    KeyDown,
    Paint,
}

pub trait Application {

}

#[derive(Debug)]
#[repr(C)]
pub struct Window<T: Application> {
    application: T,
    instance: HINSTANCE,
    handle: HWND,
    pub info: Option<String>
}

impl <T: Application> Window<T> {
    fn check_reference(&self) -> bool{
        true
    }

    pub fn new(title: &str, window_class_name: &str, app: T) -> Result<Box<Self>> {
        let hinstance = unsafe { GetModuleHandleW(None).ok() }?;
        // println!("hinstance = {}", &hinstance.0);

        REGISTER_WINDOW_CLASS.call_once(|| {
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                hInstance: hinstance, // A handle to the process that contains the window procedure
                style: CS_HREDRAW | CS_VREDRAW, // Styling
                hIcon: unsafe { LoadIconW(None, IDI_EXCLAMATION) },
                hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() }, // A handle to the class cursor
                hbrBackground: unsafe { CreateSolidBrush(BGCOLOUR).ok().unwrap() },
                lpszClassName: str_to_pcwstr(window_class_name),
                lpfnWndProc: Some(Self::wndproc), // A pointer to the window procedure - defined below
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassExW(&wc) }, 0);
        });

        let mut window = Box::new(Self {
            application: app,
            instance: hinstance,
            handle: HWND(0),
            info: None
        });

        // println!("RawPtr: {}", &mut window as *mut c_void as u32);

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
                // we then cast this raw pointer to *const c_void type `_` as is required
                // &mut window as *mut _ as *mut c_void,
                window.as_mut() as *mut _ as _,
            )
            .ok()?
        };

        if hwnd.is_invalid() {
            return Err(unsafe {
                Error::new(GetLastError().into(), "Failed to create window".into())
            });
        } else {
            window.handle = hwnd;
        }

        Ok(window)
    }

    unsafe fn event_handler(
        &mut self,
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
                WM_NCCALCSIZE => {
                    // Stop this msg passing to the default procedure as it screws up borderless
                    LRESULT(0)
                }
                WM_PAINT => {
                    // println!("WM_PAINT");
                    ValidateRect(hwnd, std::ptr::null());
                    LRESULT(0)
                }
                WM_NCHITTEST => Region::hit_test(
                    hwnd,
                    POINT {
                        x: GET_X_LPARAM(lparam.0 as u32),
                        y: GET_Y_LPARAM(lparam.0 as u32),
                    },
                ),
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                // WM_SIZE => {
                //     println!("WM_SIZE");
                //     PostMessageW(hwnd, WM_SIZE, wparam, lparam);
                //     LRESULT(0)
                // }
                _ => LRESULT(0)
        }
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // println!("Message: {}", message as u32);
        if message as u32 == WM_NCCREATE | WM_CREATE {
            let margins = MARGINS {
                cxLeftWidth: 1,
                cxRightWidth: 1,
                cyTopHeight: 1,
                cyBottomHeight: 1,
            };
            DwmExtendFrameIntoClientArea(hwnd, &margins).unwrap();
            let cs = lparam.0 as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;
            // assert_ne!(this, 0)
            (*this).handle = hwnd;

            SetWindowLongPtrW(hwnd, GWLP_USERDATA, this as _);
        } else {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;
            if let Some(this) = this.as_mut(){
                return this.event_handler(hwnd, message, wparam, lparam);
            }
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

    #[inline]
    pub fn hwnd(&self) -> HWND {
        self.handle
    }

    #[inline]
    pub fn hinstance(&self) -> HINSTANCE {
        self.instance
    }

    pub fn get_size(&self) -> Result<(u32, u32)> {
        let rect = get_window_rect(self.handle)?;
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok((width as u32, height as u32))
    }
}

unsafe impl<T: Application> HasRawWindowHandle for Window<T> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut hdl = Win32Handle::empty();
        hdl.hinstance = self.instance.0 as *mut c_void;
        hdl.hwnd = self.handle.0 as *mut c_void;

        RawWindowHandle::Win32(hdl)
    }
}

fn get_titlebar_height() -> i32 {
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
