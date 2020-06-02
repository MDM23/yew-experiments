use yew::prelude::*;
pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <h1>{ "Hello world!" }</h1>
        }
    }
}
