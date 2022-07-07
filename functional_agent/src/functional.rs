use yew::prelude::*;
use yew_agent::{Dispatched, use_bridge, UseBridgeHandle};
use crate::{api_agent, ApiAgent, ComponentBroker, Data};

#[function_component(Functional)]
pub fn functional() -> Html {
    let first_render = use_state(|| true);
    let api_msg = use_state(|| "initial-functional".to_string());
    {
        let api_msg = api_msg.clone();
        let _bridge: UseBridgeHandle<ComponentBroker> = use_bridge(move |x| {
            match x {
                Data::ResultA(_) => {
                    log::info!("functional component discard result from API-A");
                },
                Data::ResultB(_) => {
                    log::info!("got data from broker functional component");
                    api_msg.set("functional component got data from API-B".into());
                }
            }
        });
    }
    {
        let first_render = first_render.clone();
        use_effect( move || {
            if *first_render {
                let mut dispatcher = ApiAgent::dispatcher();
                log::info!("sending fetch request for API B");
                dispatcher.send(api_agent::Fetch::ApiB);
                first_render.set(false);
            }
            || ()
        });
    }

    let result = html! {
        <div>{api_msg.to_string()}</div>
    };
    result
}