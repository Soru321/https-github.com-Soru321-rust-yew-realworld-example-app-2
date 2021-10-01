use yew::services::fetch::FetchTask;
use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use yew_router::route::Route;
 

  use crate::routes::{home::Home,};

  pub struct App {}

  impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        
        let render = Router::render(|switch: Route| match switch {
            Route::Route => html! {<Home/>},
        });

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}