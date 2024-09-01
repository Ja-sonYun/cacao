use std::ffi::CStr;
use std::os::raw::c_char;

use core_graphics::display::CGRect;
use objc::rc::{Id, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

use crate::foundation::{id, to_bool, NSInteger, BOOL, NO, YES};
use crate::geometry::Rect;

/// Wrapper for a `NSScreen` object.
///
/// In general we strive to avoid using this in the codebase, but it's a requirement for moving
/// objects in and out of certain situations (e.g, `UserDefaults`).
#[derive(Debug)]
pub struct NSScreen(pub Id<Object, Owned>);

impl NSScreen {
    pub fn new() -> Self {
        unsafe {
            let obj = msg_send_id![class!(NSScreen), mainScreen];
            NSScreen(obj)
        }
    }

    pub fn cg_rect(&self) -> CGRect {
        unsafe {
            let frame: CGRect = msg_send![&self.0, frame];
            frame
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::from(self.cg_rect())
    }
}
