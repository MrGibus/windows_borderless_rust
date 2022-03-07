// use windows::{
//     core::*,
//     Win32::Foundation::*,
//     Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea,
//     Win32::Graphics::Gdi::ValidateRect,
//     Win32::Graphics::Gdi::*,
//     Win32::System::LibraryLoader::GetModuleHandleW,
//     Win32::UI::Controls::{GetThemeSysSize, MARGINS},
//     Win32::UI::WindowsAndMessaging::*,
// };
//
// use crate::utils::*;
// use crate::window::{Region, Window};
//
// pub enum Event {
//     Resize,
//     KeyDown,
//     Paint,
// }
//
// #[derive(Eq, PartialEq)]
// pub enum EventFlow {
//     Handled,
//     NotHandled,
// }
//
// impl EventFlow {
//     #[inline]
//     fn is_handled(&self) -> bool {
//         self == &Self::Handled
//     }
// }
//
// pub trait Application {
//     fn get_window(&self) -> &Window;
//
//     fn event_handler(&mut self, event: Event);
//
//     fn start(&mut self) {
//         let mut message = MSG::default();
//
//         unsafe {
//             while GetMessageW(&mut message, self.get_window().hwnd(), 0, 0).into() {
//                 // println!("INTERCEPTED: {:?}", &message);
//                 if self.app_message_interceptor(&message).is_handled() {
//                     println!("Continuing");
//                     continue;
//                 }
//                 TranslateMessage(&message);
//                 DispatchMessageW(&message);
//             }
//         }
//     }
//
//     // This method intercepts some messages being sent and exposes them to the event_handler
//     // FIXME: Does not capture  WM_SIZE required to determine window sizing events
//     unsafe fn app_message_interceptor(&mut self, msg: &MSG) -> EventFlow {
//         match msg.message as u32 {
//             WM_KEYDOWN => {
//                 println!("A button was pressed");
//                 self.event_handler(Event::KeyDown);
//                 println!("data from app: {:?}", self.get_window().info);
//                 EventFlow::NotHandled
//             }
//             WM_PAINT => {
//                 self.event_handler(Event::Paint);
//                 EventFlow::NotHandled
//             }
//             _ => EventFlow::NotHandled,
//         }
//     }
// }
