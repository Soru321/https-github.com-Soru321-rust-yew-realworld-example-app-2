// src/pages/home.rs
use yew::prelude::*;

// My home page Struct
pub struct Home {}

//componante for home page impl msg and proprieties
impl Component for Home {
    type Message = ();
    type Properties = ();
     
    // create this proprieties
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    //msg update
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    //fn change proprietes
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    // fn view
    fn view(&self) -> Html {
        html! { <span>{"rust-yew-realworld-example-app Sweet Home!"}</span> }
    }
}