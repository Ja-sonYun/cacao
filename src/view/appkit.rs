//! This module does one specific thing: register a custom `NSView` class that's... brought to the
//! modern era.
//!
//! I kid, I kid.
//!
//! It just enforces that coordinates are judged from the top-left, which is what most people look
//! for in the modern era. It also implements a few helpers for things like setting a background
//! color, and enforcing layer backing by default.

use objc::declare::ClassDecl;
use objc::foundation::NSRect;
use objc::rc::{Id, Owned};
use objc::runtime::{Bool, Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::dragdrop::DragInfo;
use crate::foundation::{id, load_or_register_class, nil, NSUInteger};
use crate::utils::load;
use crate::view::{ViewDelegate, BACKGROUND_COLOR, VIEW_DELEGATE_PTR};

/// Enforces normalcy, or: a needlessly cruel method in terms of the name. You get the idea though.
extern "C" fn enforce_normalcy(_: &Object, _: Sel) -> Bool {
    return Bool::YES;
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_entered<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = load::<T>(this, VIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::retain(info).unwrap() },
    })
    .into()
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn prepare_for_drag_operation<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) -> Bool {
    let view = load::<T>(this, VIEW_DELEGATE_PTR);

    Bool::new(view.prepare_for_drag_operation(DragInfo {
        info: unsafe { Id::retain(info).unwrap() },
    }))
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn perform_drag_operation<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) -> Bool {
    let view = load::<T>(this, VIEW_DELEGATE_PTR);

    Bool::new(view.perform_drag_operation(DragInfo {
        info: unsafe { Id::retain(info).unwrap() },
    }))
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn conclude_drag_operation<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, VIEW_DELEGATE_PTR);

    view.conclude_drag_operation(DragInfo {
        info: unsafe { Id::retain(info).unwrap() },
    });
}

/// Called when a drag/drop operation has entered this view.
extern "C" fn dragging_exited<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) {
    let view = load::<T>(this, VIEW_DELEGATE_PTR);

    view.dragging_exited(DragInfo {
        info: unsafe { Id::retain(info).unwrap() },
    });
}

/// Called for layer updates.
extern "C" fn update_layer(this: &Object, _: Sel) {
    unsafe {
        let background_color: id = *this.get_ivar(BACKGROUND_COLOR);

        if background_color != nil {
            let layer: id = msg_send![this, layer];
            let cg: id = msg_send![background_color, CGColor];
            let _: () = msg_send![layer, setBackgroundColor: cg];
        }
    }
}

extern "C" fn draw_rect(this: &Object, _: Sel, rect: NSRect) {
    let view: id = unsafe { msg_send![this, view] };
    let delegate: id = unsafe { msg_send![view, delegate] };

    if delegate != nil {
        let _: () = unsafe { msg_send![delegate, drawRect: rect] };
    }
}

/// Injects an `NSView` subclass. This is used for the default views that don't use delegates - we
/// have separate classes here since we don't want to waste cycles on methods that will never be
/// used if there's no delegates.
pub(crate) fn register_view_class() -> &'static Class {
    load_or_register_class("NSView", "RSTView", |decl| unsafe {
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(_, _) -> _);
        decl.add_method(sel!(updateLayer), update_layer as extern "C" fn(_, _));
        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern "C" fn(_, _) -> _);

        decl.add_ivar::<id>(BACKGROUND_COLOR);
    })
}

/// Injects an `NSView` subclass, with some callback and pointer ivars for what we
/// need to do.
pub(crate) fn register_view_class_with_delegate<T: ViewDelegate>(instance: &T) -> &'static Class {
    load_or_register_class("NSView", instance.subclass_name(), |decl| unsafe {
        // A pointer to the ViewDelegate instance on the Rust side.
        // It's expected that this doesn't move.
        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);
        decl.add_ivar::<id>(BACKGROUND_COLOR);

        decl.add_method(sel!(updateLayer), update_layer as extern "C" fn(_, _));

        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern "C" fn(_, _) -> _);

        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(_, _) -> _);

        decl.add_method(sel!(drawRect:), draw_rect as extern "C" fn(_, _, _) -> _);

        // Drag and drop operations (e.g, accepting files)
        decl.add_method(sel!(draggingEntered:), dragging_entered::<T> as extern "C" fn(_, _, _) -> _);

        decl.add_method(
            sel!(prepareForDragOperation:),
            prepare_for_drag_operation::<T> as extern "C" fn(_, _, _) -> _,
        );

        decl.add_method(
            sel!(performDragOperation:),
            perform_drag_operation::<T> as extern "C" fn(_, _, _) -> _,
        );

        decl.add_method(
            sel!(concludeDragOperation:),
            conclude_drag_operation::<T> as extern "C" fn(_, _, _),
        );

        decl.add_method(sel!(draggingExited:), dragging_exited::<T> as extern "C" fn(_, _, _));
    })
}
