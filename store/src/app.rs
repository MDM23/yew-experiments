use super::state::{Input, ReadOnly, State};
use crate::reduce;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: ReadOnly<State>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: ReadOnly<State>,
}

pub enum Msg {
    UpdatePayload(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            state: props.state,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePayload(p) => reduce!(Input::SetText(p)),
        }
        true
    }

    fn view(&self) -> Html {
        let oninput = self
            .link
            .callback(|e: InputData| Msg::UpdatePayload(e.value));
        html! {
            <>
                <input oninput=oninput value=self.state.borrow().text.clone() />
                <Sub text=self.state.borrow().text.clone() />
                <Sub text=self.state.borrow().text.clone() />
                <Sub text=self.state.borrow().text.clone() />
            </>
        }
    }
}

pub struct Sub {
    text: String,
}

#[derive(Clone, Properties)]
pub struct SubProps {
    text: String,
}

impl Component for Sub {
    type Message = ();
    type Properties = SubProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { text: props.text }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.text = props.text;
        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>{ format!("State value = {}", self.text) }</div>
        }
    }
}
