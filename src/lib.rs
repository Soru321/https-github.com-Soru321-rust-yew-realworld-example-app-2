extern crate wasm_bindgen;
extern crate yew;

mod pages;
use pages::Home;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Rust test generate automaticly no using in my app
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/


/*
// hello Stratu for bigining my rust-yew-realworld-example-app
struct Hello {}

//implement Hello with componanate msg & proprety
impl Component for Hello {
    type Message = ();
    type Properties = ();
   
    // fn create 
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    // update fn
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
     
    // chage fn
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    //fn view 
    fn view(&self) -> Html {
        html! { <span>{"Hello rust-yew-realworld-example-app!"}</span> }
    }
}
*/

#[wasm_bindgen(start)]
//entry point of my rust-yew-realworld-example-app
pub fn run_app() {
    //App::<Hello>::new().mount_to_body();
    App::<Home>::new().mount_to_body();
}