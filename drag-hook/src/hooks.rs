use gloo::events::{EventListener, EventListenerOptions};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, MouseEvent};
use yew::{Callback, NodeRef};
use yew_functional::{use_hook, use_reducer, use_ref, Hook};

type Shared<T> = Rc<RefCell<T>>;
pub struct Coordinates(pub i32, pub i32);

#[derive(Default)]
struct DragListeners {
    dragstart: Option<EventListener>,
    mouseup: Option<EventListener>,
    mouseleave: Option<EventListener>,
    mousemove: Option<EventListener>,
}

#[derive(Clone, Default)]
struct ListenerRef(Shared<DragListeners>);

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

pub enum DragAction {
    Dragged(HtmlElement),
    Dropped,
}

#[derive(Clone)]
pub struct DragHookOptions {
    pub event_callback: Callback<DragAction>,
    pub reset_on_drop: bool,
}

impl Default for DragHookOptions {
    fn default() -> Self {
        Self {
            event_callback: Callback::noop(),
            reset_on_drop: false,
        }
    }
}

pub fn use_drag() -> (Rc<Coordinates>, Rc<RefCell<NodeRef>>) {
    use_drag_with_options(Default::default())
}

pub fn use_drag_with_options(options: DragHookOptions) -> (Rc<Coordinates>, Rc<RefCell<NodeRef>>) {
    #[derive(Default)]
    struct UseDragState {
        listeners: ListenerRef,
    }

    impl Hook for UseDragState {
        fn tear_down(&mut self) {
            self.listeners.unregister();
        }
    }

    enum Action {
        Translate(Coordinates),
        Reset,
    }

    let node = use_ref(|| NodeRef::default());
    let node_c = node.clone();

    let (coords, reduce) = use_reducer(
        |prev: Rc<Coordinates>, action: Action| match action {
            Action::Translate(delta) => Coordinates(prev.0 + delta.0, prev.1 + delta.1),
            Action::Reset => Coordinates(0, 0),
        },
        Coordinates(0, 0),
    );

    use_hook(
        move |_state: &mut UseDragState, hook_callback| {
            hook_callback(
                move |state: &mut UseDragState| {
                    if let Some(element) = node_c.borrow().cast::<HtmlElement>() {
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
                                    options
                                        .event_callback
                                        .emit(DragAction::Dragged(element_c.clone()));

                                    let body = &web_sys::window()
                                        .unwrap()
                                        .document()
                                        .unwrap()
                                        .body()
                                        .unwrap();

                                    let l1 = listeners.clone();
                                    let r1 = reduce.clone();
                                    let op1 = options.clone();
                                    listeners.set_mouseup(EventListener::once(
                                        &body,
                                        "mouseup",
                                        move |_| {
                                            l1.drag_stop();

                                            if op1.reset_on_drop {
                                                r1(Action::Reset);
                                            }

                                            op1.event_callback.emit(DragAction::Dropped);
                                        },
                                    ));

                                    let l2 = listeners.clone();
                                    let r2 = reduce.clone();
                                    let op2 = options.clone();
                                    listeners.set_mouseleave(EventListener::once(
                                        &body,
                                        "mouseleave",
                                        move |_| {
                                            l2.drag_stop();

                                            if op2.reset_on_drop {
                                                r2(Action::Reset);
                                            }

                                            op2.event_callback.emit(DragAction::Dropped);
                                        },
                                    ));

                                    let mut last_coords = e
                                        .dyn_ref::<MouseEvent>()
                                        .map(|m: &MouseEvent| (m.client_x(), m.client_y()))
                                        .unwrap();

                                    let reduce = reduce.clone();

                                    listeners.set_mousemove(EventListener::new(
                                        &body,
                                        "mousemove",
                                        move |ev: &Event| {
                                            let new_coords = ev
                                                .dyn_ref::<MouseEvent>()
                                                .map(|m: &MouseEvent| (m.client_x(), m.client_y()))
                                                .unwrap();

                                            reduce(Action::Translate(Coordinates(
                                                new_coords.0 - last_coords.0,
                                                new_coords.1 - last_coords.1,
                                            )));

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
        || Default::default(),
    );

    (coords, node)
}
