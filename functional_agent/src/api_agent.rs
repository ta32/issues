use yew_agent::{Agent, AgentLink, Context, Dispatched, Dispatcher, HandlerId};
use crate::component_broker;
use crate::component_broker::ComponentBroker;

pub enum Fetch {
    ApiA,
    ApiB
}

pub enum Msg {
    ResultA(String),
    ResultB(String)

}

pub struct ApiAgent {
    link: AgentLink<ApiAgent>,
    dispatcher: Dispatcher<ComponentBroker>
}

async fn fetch_data() -> String {
    "Some data".to_string()
}

impl Agent for ApiAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Fetch;
    type Output = ();

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            dispatcher: ComponentBroker::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        log::info!("send msg to broker");
        match msg {
            Msg::ResultA(body) => {
                self.dispatcher.send(component_broker::Data::ResultA(body))
            }
            Msg::ResultB(body) => {
                self.dispatcher.send(component_broker::Data::ResultB(body))
            }
        }
    }

    fn handle_input(&mut self, input: Self::Input, _id: HandlerId) {
        log::info!("fetch data request");
        match input {
            Fetch::ApiA => {
                self.link.send_future( async move {
                    let body = fetch_data().await;
                    Msg::ResultA(body)
                })
            },
            Fetch::ApiB => {
                self.link.send_future( async move {
                    let body = fetch_data().await;
                    Msg::ResultB(body)
                })
            }
        }
    }
}
