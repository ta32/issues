use std::collections::HashSet;
use yew_agent::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    ResultA(String),
    ResultB(String)
}

pub struct ComponentBroker {
    link: AgentLink<ComponentBroker>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ComponentBroker {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Data;
    type Output = Data;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        log::info!("connected");
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        log::info!("handle_input");
        match msg {
            Data::ResultA(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, Data::ResultA(s.clone()));
                }
            },
            Data::ResultB(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, Data::ResultB(s.clone()));
                }
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        log::info!("disconnected");
        self.subscribers.remove(&id);
    }
}
