use crate::foundation::{id, nil, NSInteger, NSPoint, NSString};
use objc::ffi::YES;
use objc::rc::{Id, Owned, Shared};
use objc::runtime::Object;
use objc::{class, msg_send, msg_send_id, sel};

#[derive(Debug)]
pub struct Timer {
    pub objc: Id<Object, Owned>,
    // pub timer: Option<DispatchSourceTimer>,
}

impl Timer {
    pub unsafe fn new() -> Self {
        let callback_selector = sel!(timerCallback:);

        let interval = 1.0;
        let timer: id = msg_send![
            class!(NSTimer),
            scheduledTimerWithTimeInterval: interval,
            target: nil,
            selector: callback_selector,
            userInfo: nil,
            repeats: YES
        ];

        let objc: Id<Object, Owned> = Id::new(timer).unwrap();

        let run_loop: id = msg_send![class!(NSRunLoop), mainRunLoop];
        let _: () = msg_send![run_loop, addTimer: timer forMode: nil];

        Self { objc }
    }
}
