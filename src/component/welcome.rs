use gloo::timers::callback::Timeout;
use yew::prelude::*;

pub struct Welcome {
    now: String,
}

pub enum WelcomeMsg {
    RefreshTime,
}

fn get_now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

impl Component for Welcome {
    type Message = WelcomeMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { now: get_now() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WelcomeMsg::RefreshTime => {
                let link = ctx.link().clone();
                self.now = get_now();
                Timeout::new(1000, move || link.send_message(WelcomeMsg::RefreshTime)).forget();
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(WelcomeMsg::RefreshTime)
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="search-container">
                <p><b>{"Welcome to Pied Piper!"}</b></p>
                <hr/>
                <p>{self.now.clone()}</p>
            </div>
        }
    }
}
