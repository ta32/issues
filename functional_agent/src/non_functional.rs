use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
use crate::{ApiAgent, ComponentBroker, Data};
use crate::api_agent::Fetch;
use crate::non_functional::Msg::Discard;

pub enum Msg {
    GotData(String),
    Discard
}

pub struct NonFunctional {
    _sub: Box<dyn Bridge<ComponentBroker>>,
    api_dispatcher: Dispatcher<ApiAgent>,
    data: String
}

impl Component for NonFunctional {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _sub: ComponentBroker::bridge(ctx.link().callback(|x| {
                match x {
                    Data::ResultA(_) => Msg::GotData("API Data".into()),
                    _ => Discard
                }
            })),
            api_dispatcher: ApiAgent::dispatcher(),
            data: "initial-struct component".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotData(_) => {
               self.data = "struct component got data from API-A".into();
               true
            }
            _ => {
                log::info!("struct component discard data from API-B");
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let result = html! {
            <div>{self.data.clone()}</div>
        };
        result
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.api_dispatcher.send(Fetch::ApiA);
        }
    }
}