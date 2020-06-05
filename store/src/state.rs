use super::app::App;
use std::cell::RefCell;
use std::convert::identity;
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

thread_local! {
    pub static STATE: RefCell<Callback<Input>> = RefCell::new(Callback::noop());
}

#[macro_export]
macro_rules! reduce {
    ($cmd: expr) => {
        crate::state::STATE.with(|cb| {
            cb.borrow().emit($cmd);
        })
    };
}

#[derive(Debug)]
pub enum Input {
    SetText(String),
}

#[derive(Default)]
pub struct State {
    pub text: String,
}

#[derive(Debug)]
pub struct ReadOnly<S> {
    state: Rc<RefCell<S>>,
}

impl<S> ReadOnly<S> {
    pub fn borrow<'a>(&'a self) -> impl Deref<Target = S> + 'a {
        self.state.borrow()
    }
}

impl<S> Clone for ReadOnly<S> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

pub struct StateContainer {
    state: Rc<RefCell<State>>,
}

impl Component for StateContainer {
    type Message = Input;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        STATE.with(|cb| cb.replace(link.callback(identity)));

        Self {
            state: Rc::new(RefCell::new(State {
                text: String::from("test"),
            })),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state.borrow_mut().reduce(msg);
        true
    }

    fn view(&self) -> Html {
        html! {
            <App state=ReadOnly { state: self.state.clone() } />
        }
    }
}

impl State {
    fn reduce(&mut self, msg: Input) {
        match msg {
            Input::SetText(t) => {
                self.text = t;
            }
        }
    }
}
