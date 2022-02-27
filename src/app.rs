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
use crate::window::{Region, Window};

pub enum Event {
    KeyDown,
    Paint,
}

#[derive(Eq, PartialEq)]
pub enum EventFlow {
    Handled,
    NotHandled,
}

impl EventFlow {
    #[inline]
    fn is_handled(&self) -> bool {
        self == &Self::Handled
    }
}

pub trait Application {
    fn get_window(&self) -> &Window;

    fn event_handler(&mut self, event: Event);

    fn start(&mut self) {
        let mut message = MSG::default();

        unsafe {
            while GetMessageW(&mut message, self.get_window().hwnd(), 0, 0).into() {
                if self.app_message_interceptor(&message).is_handled() {
                    continue;
                }
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }

    // This method intercepts some messages being sent and exposes them to the event_handler
    unsafe fn app_message_interceptor(&mut self, msg: &MSG) -> EventFlow {
        match msg.message as u32 {
            WM_KEYDOWN => {
                println!("A button was pressed");
                self.event_handler(Event::KeyDown);
                EventFlow::Handled
            }
            WM_PAINT => {
                self.event_handler(Event::Paint);
                EventFlow::NotHandled
            }
            _ => EventFlow::NotHandled,
        }
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message as u32 {
            WM_NCCALCSIZE => {
                // Stop this msg passing to the default procedure as it screws up borderless
                LRESULT(0)
            }
            WM_CREATE => {
                let margins = MARGINS {
                    cxLeftWidth: 1,
                    cxRightWidth: 1,
                    cyTopHeight: 1,
                    cyBottomHeight: 1,
                };
                DwmExtendFrameIntoClientArea(hwnd, &margins).unwrap();
                println!("WM_CREATE");
                LRESULT(0)
            }
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(hwnd, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_NCHITTEST => Region::hit_test(
                hwnd,
                POINT {
                    x: GET_X_LPARAM(lparam.0 as u32),
                    y: GET_Y_LPARAM(lparam.0 as u32),
                },
            ),
            _ => DefWindowProcW(hwnd, message, wparam, lparam),
        }
    }
}
