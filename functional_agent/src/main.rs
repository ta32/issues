extern crate core;

mod component_broker;
mod api_agent;
mod non_functional;
mod functional;

use yew::prelude::*;
use non_functional::NonFunctional;
use functional::Functional;
use crate::api_agent::ApiAgent;
use crate::component_broker::{ComponentBroker, Data};

#[function_component(Model)]
pub fn model() -> Html {
    // example works if i had the non-functional component
    // <div>
    //     <div>{"Hello, world!"}</div>
    //     <NonFunctional/>
    //     <Functional />
    // </div>
    // not working example
    let result = html! {
        <div>
            <div>{"Hello, world!"}</div>
            <Functional />
        </div>
    };
    result
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}