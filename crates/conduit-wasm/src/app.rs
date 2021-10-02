use yew::services::fetch::FetchTask;
use yew::{agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use yew_router::route::Route;
 

  use crate::routes::{home::Home, AppRoute};

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
        let routes = AppRoute::routes (|switch : AppRoute| match switch {
            AppRoute::Home => html!{<Home/>} 
        });

        html!{<AppRoutes > AppRoute, ()> routes = routes}
       
    }
}