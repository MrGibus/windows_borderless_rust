use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*, Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea,
};
use windows::Win32::UI::Controls::MARGINS;

mod utils;
use utils::rgb;
use crate::utils::str_to_pcwstr;

const TITLE: &str = "My window";
const CLASS: &str = "window";

/// Default background colour
const BGCOLOUR: u32 = rgb(52, 55, 60);


fn main() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(None);
        debug_assert!(hinstance.0 != 0);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            hInstance: hinstance, // A handle to the process that contains the window procedure
            style: CS_HREDRAW | CS_VREDRAW, // Styling
            hCursor: LoadCursorW(None, IDC_ARROW), // A handle to the class cursor
            hbrBackground: CreateSolidBrush(BGCOLOUR),
            lpszClassName: str_to_pcwstr(CLASS),
            lpfnWndProc: Some(wndproc),     // A pointer to the window procedure - defined below
            ..Default::default()
        };

        let atom = RegisterClassExW(&wc);
        debug_assert!(atom != 0);

        CreateWindowExW(
            Default::default(),
            CLASS,
            TITLE,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            hinstance,
            std::ptr::null(),
        );

        let mut message = MSG::default();

        while GetMessageW(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageW(&message);
        }

        Ok(())
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
