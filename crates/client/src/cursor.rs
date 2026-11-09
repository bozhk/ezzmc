//! Курсор мыши по запросам виджетов. VANILLA: GuiGraphicsExtractor#requestCursor + Window#selectCursor
//! (при выключенной опции allowCursorChanges всегда стрелка).
use std::sync::atomic::{AtomicU8, Ordering};
use winit::window::CursorIcon;

/// VANILLA: CursorTypes.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)] // ResizeNs/ResizeAll — для скролла и перетаскивания в экранах, которые ещё не просят курсор
pub enum Kind {
    Arrow = 0,
    Ibeam,
    PointingHand,
    ResizeNs,
    ResizeEw,
    ResizeAll,
    NotAllowed,
}

static REQUESTED: AtomicU8 = AtomicU8::new(0);

/// Виджет под мышью просит курсор (последний запрос кадра побеждает).
pub fn request(k: Kind) {
    REQUESTED.store(k as u8, Ordering::Relaxed);
}

/// Забирает запрос кадра и сбрасывает его в стрелку.
pub fn take(allow: bool) -> CursorIcon {
    let k = REQUESTED.swap(0, Ordering::Relaxed);
    if !allow {
        return CursorIcon::Default;
    }
    match k {
        1 => CursorIcon::Text,
        2 => CursorIcon::Pointer,
        3 => CursorIcon::NsResize,
        4 => CursorIcon::EwResize,
        5 => CursorIcon::AllScroll,
        6 => CursorIcon::NotAllowed,
        _ => CursorIcon::Default,
    }
}
