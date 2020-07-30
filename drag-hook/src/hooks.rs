use gloo::events::{EventListener, EventListenerOptions};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, MouseEvent};
use yew::{Callback, NodeRef};
use yew_functional::{use_hook, use_reducer, Hook};

pub struct Coordinates(pub i32, pub i32);

#[derive(Debug)]
struct DragListeners {
    dragstart: Option<EventListener>,
    mouseup: Option<EventListener>,
    mouseleave: Option<EventListener>,
    mousemove: Option<EventListener>,
}

struct ListenerRef(Rc<RefCell<DragListeners>>);

impl Default for ListenerRef {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(DragListeners {
            dragstart: None,
            mouseup: None,
            mouseleave: None,
            mousemove: None,
        })))
    }
}

impl Clone for ListenerRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ListenerRef {
    pub fn set_dragstart(&self, l: EventListener) {
        self.0.borrow_mut().dragstart = Some(l);
    }

    pub fn set_mouseup(&self, l: EventListener) {
        self.0.borrow_mut().mouseup = Some(l);
    }

    pub fn set_mouseleave(&self, l: EventListener) {
        self.0.borrow_mut().mouseleave = Some(l);
    }

    pub fn set_mousemove(&self, l: EventListener) {
        self.0.borrow_mut().mousemove = Some(l);
    }

    pub fn drag_stop(&self) {
        let mut s = self.0.borrow_mut();
        s.mouseleave.take();
        s.mousemove.take();
        s.mouseup.take();
    }

    pub fn unregister(&self) {
        self.drag_stop();
        self.0.borrow_mut().dragstart.take();
    }
}

#[derive(Debug)]
pub enum DragAction {
    Dragging(HtmlElement),
    Dropped,
}

pub fn use_drag(
    trigger: NodeRef,
    callback: Callback<DragAction>,
    initial: Coordinates,
) -> Rc<Coordinates> {
    struct UseDragState {
        listeners: ListenerRef,
    }

    impl Hook for UseDragState {
        fn tear_down(&mut self) {
            self.listeners.unregister();
        }
    }

    let (coords, move_by) = use_reducer(
        |prev: Rc<Coordinates>, delta: Coordinates| Coordinates(prev.0 + delta.0, prev.1 + delta.1),
        initial,
    );

    use_hook(
        move |_state: &mut UseDragState, hook_callback| {
            hook_callback(
                move |state: &mut UseDragState| {
                    if let Some(element) = trigger.cast::<HtmlElement>() {
                        element.set_draggable(true);
                        let listeners = state.listeners.clone();
                        let element_c = element.clone();
                        state
                            .listeners
                            .set_dragstart(EventListener::new_with_options(
                                &element,
                                "dragstart",
                                EventListenerOptions::enable_prevent_default(),
                                move |e: &Event| {
                                    e.prevent_default();
                                    callback.emit(DragAction::Dragging(element_c.clone()));

                                    let body = &web_sys::window()
                                        .unwrap()
                                        .document()
                                        .unwrap()
                                        .body()
                                        .unwrap();

                                    let l1 = listeners.clone();
                                    let cb1 = callback.clone();
                                    listeners.set_mouseup(EventListener::once(
                                        &body,
                                        "mouseup",
                                        move |_| {
                                            l1.drag_stop();
                                            cb1.emit(DragAction::Dropped);
                                        },
                                    ));

                                    let l2 = listeners.clone();
                                    let cb2 = callback.clone();
                                    listeners.set_mouseleave(EventListener::once(
                                        &body,
                                        "mouseleave",
                                        move |_| {
                                            l2.drag_stop();
                                            cb2.emit(DragAction::Dropped);
                                        },
                                    ));

                                    let mut last_coords = e
                                        .dyn_ref::<MouseEvent>()
                                        .map(|m: &MouseEvent| (m.client_x(), m.client_y()))
                                        .unwrap();

                                    let m = move_by.clone();

                                    listeners.set_mousemove(EventListener::new(
                                        &body,
                                        "mousemove",
                                        move |ev: &Event| {
                                            let new_coords = ev
                                                .dyn_ref::<MouseEvent>()
                                                .map(|m: &MouseEvent| (m.client_x(), m.client_y()))
                                                .unwrap();

                                            m(Coordinates(
                                                new_coords.0 - last_coords.0,
                                                new_coords.1 - last_coords.1,
                                            ));

                                            last_coords = new_coords;
                                        },
                                    ));
                                },
                            ));
                    }

                    false
                },
                true,
            );
        },
        || UseDragState {
            listeners: Default::default(),
        },
    );

    coords
}
