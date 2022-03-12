use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::utils::{rgb, str_to_pcwstr, GET_X_LPARAM, GET_Y_LPARAM};
use crate::input::KeyCode;
use crate::winapi_utils::*;

// use crate::State;
use crate::render::{State, Input};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, Win32Handle};
use std::ffi::c_void;
use windows::Win32::Foundation::LRESULT;

/// Default background colour
const BGCOLOUR: u32 = rgb(52, 55, 60);

#[derive(Debug)]
#[repr(C)]
pub struct Window {
    // Raw pointer to the WGPU struct
    data: *mut State,
    instance: HINSTANCE,
    handle: HWND,
}

impl Window {
    #[allow(dead_code)]
    fn test_correct(&self) {
        println!(
            "I'm a little window, short and stdout. \
        This is my handle: {:?} this is my instance {:?}",
            &self.handle, &self.instance
        )
    }

    pub fn new(title: &str, window_class_name: &str) -> Result<Box<Self>> {
        let hinstance = get_current_module_handle()?;

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            hInstance: hinstance, // A handle to the process that contains the window procedure
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC, // Styling (Nothing to do with aesthetics)
            hIcon: warning_icon()?,
            hCursor: default_cursor()?, // A handle to the class cursor
            hbrBackground: solid_brush(BGCOLOUR)?,
            lpszClassName: str_to_pcwstr(window_class_name),

            lpfnWndProc: Some(Self::wnd_proc_sys), // A pointer to the window procedure - defined below
            ..Default::default()
        };

        let _atom = register_window_class(&wc)?;

        let mut window = Box::new(Self {
            data: std::ptr::null_mut(),
            instance: hinstance,
            handle: HWND(0),
        });

        let _hwnd = create_window(
            Default::default(),
            window_class_name,
            title,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            [CW_USEDEFAULT; 2],
            [CW_USEDEFAULT; 2],
            hinstance,
            // mutable reference to raw pointer where we let the compiler work out the type '_'
            // we then cast this raw pointer to *mut c_void type `_` as is required
            // &mut window as *mut _ as *mut c_void,
            window.as_mut() as *mut _ as _,
        )?;

        Ok(window)
    }

    pub fn set_state(&mut self, state: &mut State) {
        self.data = state
    }

    pub fn get_size(&self) -> Result<(u32, u32)> {
        let rect = get_window_rect(self.handle)?;
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok((width as u32, height as u32))
    }

    fn wnd_proc(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
        match message {
            WM_CREATE => {
                extend_frame_into_client_area(
                    self.handle,
                    &WindowStyle::Borderless
                ).unwrap();
                None
            }
            WM_DESTROY => {
                post_quit_message(0);
                None
            }
            WM_PAINT => unsafe {
                validate_rect(self.handle).unwrap();

                if let Some(state) = self.data.as_mut() {
                    state.render().unwrap();
                }

                None
            },
            WM_SIZE | WM_SIZING => unsafe {
                let size = self.get_size().unwrap();

                if let Some(state) = self.data.as_mut() {
                    state.resize(size);
                }
                None
            },
            WM_NCCALCSIZE => {
                // Stop this msg passing to the default procedure as it screws up borderless
                Some(LRESULT(0))
            }
            // Non-client hit test
            WM_NCHITTEST => Some(Region::hit_test(
                self.handle,
                POINT {
                    x: GET_X_LPARAM(lparam.0 as u32),
                    y: GET_Y_LPARAM(lparam.0 as u32),
                },
            )),
            WM_LBUTTONDOWN => {
                // println!("WM_LBUTTONDOWN {:?}, {:?}", wparam, lparam);
                let x = GET_X_LPARAM(lparam.0 as u32);
                let y = GET_Y_LPARAM(lparam.0 as u32);
                println!("Mouse Clicked at x: {:?}, y: {:?}", x, y);
                unsafe {
                    if let Some(state) = self.data.as_mut(){
                        state.input(Input::LeftClick((x as u32, y as u32)));
                    }
                }
                None
            }
            WM_MOUSEMOVE => {
                let x = GET_X_LPARAM(lparam.0 as u32);
                let y = GET_Y_LPARAM(lparam.0 as u32);
                // println!("Mouse Moved x: {:?}, y: {:?}", x, y);
                unsafe {
                    if let Some(state) = self.data.as_mut(){
                        state.input(Input::MouseMove((x as u32, y as u32)));
                    }
                }
                None
            }
            WM_KEYDOWN => {
                // println!("WM_KEYDOWN{:?}", wparam);
                // Check if Q is pressed
                if let Some(key) = KeyCode::from_raw(wparam.0) {
                    // println!("key = {:?}", key);
                    if key == KeyCode::Escape {
                        post_quit_message(0);
                        return  Some(LRESULT(0));
                    }
                    else {
                        unsafe {
                            if let Some(state) = self.data.as_mut(){
                                state.input(Input::KeyDown(key));
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
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

        let l_result = if message == WM_NCCREATE {
            // Set Window pointer
            let cs = lparam.0 as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;
            (*this).handle = hwnd;
            if hwnd.is_invalid() {
                panic!("Cannot recover: Window handle is invalid");
            }
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, this as _);
            None
        } else {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;
            if let Some(this) = this.as_mut() {
                this.wnd_proc(message, wparam, lparam)
            } else {
                None
            }
        };

        if let Some(l) = l_result {
            return l;
        }

        DefWindowProcW(hwnd, message, wparam, lparam)
    }

    pub fn start(&self) {
        let mut message = MSG::default();
        unsafe {
            // Note: Pass null because passing the window handle will not pick up all messages
            while GetMessageW(&mut message, HWND(0), 0, 0).into() {
                if message.message == WM_QUIT {
                    // Where we receive a quit message we take the wParam and use that as an exit
                    // code for our application
                    std::process::exit(message.wParam.0 as i32);
                }
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
        let border = get_border();

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
