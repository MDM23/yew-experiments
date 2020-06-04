use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

pub struct State {
    payload: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            payload: String::from("Test"),
        }
    }
}

pub type StateRef = Rc<RefCell<State>>;

pub struct App {
    link: ComponentLink<Self>,
    state: StateRef,
}

pub enum Msg {
    UpdatePayload(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            state: Rc::new(RefCell::new(State::default())),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePayload(p) => self.state.borrow_mut().payload = p,
        }
        true
    }

    fn view(&self) -> Html {
        let oninput = self
            .link
            .callback(|e: InputData| Msg::UpdatePayload(e.value));
        html! {
            <>
                <input oninput=oninput value=self.state.borrow().payload />
                <Sub state=self.state.clone() />
                <Sub state=self.state.clone() />
                <Sub state=self.state.clone() />
            </>
        }
    }
}

pub struct Sub {
    payload: String,
}

#[derive(Clone, Properties)]
pub struct SubProps {
    state: StateRef,
}

impl Component for Sub {
    type Message = ();
    type Properties = SubProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            payload: props.state.borrow().payload.clone(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.state.borrow().payload != self.payload {
            self.payload = props.state.borrow().payload.clone();
            true
        } else {
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>{ format!("State value = {}", self.payload) }</div>
        }
    }
}
