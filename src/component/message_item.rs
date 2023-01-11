use gloo::timers::callback::Timeout;
use uuid::Uuid;
use yew::prelude::*;
use yew::Properties;

pub struct MessageItem;

#[derive(Clone, PartialEq, Properties)]
pub struct MessageItemProps {
    pub value: MessageItemValue,
    pub on_close: Callback<u128>,
    pub on_open_dialog: Callback<u128>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MessageItemValue {
    pub id: u128,
    pub from_id: Option<String>,
    pub from: Option<String>,
    pub content: String,
    pub room: String,
    pub r#type: MessageItemType,
    // seconds
    pub timeout: Option<u32>,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum MessageItemType {
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl MessageItemValue {
    pub fn new(
        r#type: MessageItemType,
        room: &str,
        content: &str,
        timeout: Option<u32>,
        from: Option<&str>,
        from_id: Option<&str>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            r#type: r#type,
            room: room.to_string(),
            content: content.to_string(),
            from_id: from_id.and_then(|x| Some(x.to_string())),
            from: from.and_then(|x| Some(x.to_string())),
            timeout: timeout,
        }
    }
}

pub enum MessageItemMsg {
    Close(u128),
    OpenDialog(u128),
}

impl Component for MessageItem {
    type Message = MessageItemMsg;

    type Properties = MessageItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessageItemMsg::Close(id) => {
                ctx.props().on_close.emit(id);
                false
            }
            MessageItemMsg::OpenDialog(id) => {
                if id != 0 {
                    ctx.props().on_open_dialog.emit(id);
                }
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let value = &ctx.props().value;
        let id = value.id;
        if let Some(timeout) = value.timeout {
            let link = ctx.link().clone();
            Timeout::new(1000 * timeout, move || {
                link.send_message(MessageItemMsg::Close(id))
            })
            .forget();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value = &ctx.props().value;
        let t = format!("{:?}", &value.r#type).to_lowercase();
        let content = value.content.trim_matches('"');
        let content = match &value.from {
            Some(name) => format!("{name}: {content}"),
            None => content.to_string(),
        };
        let id = value.id;
        let style = if value.r#type == MessageItemType::Primary {
            "cursor: pointer"
        } else {
            ""
        };
        let onclick = if value.r#type == MessageItemType::Primary {
            ctx.link().callback(move |_| MessageItemMsg::OpenDialog(id))
        } else {
            ctx.link().callback(move |_| MessageItemMsg::OpenDialog(0))
        };
        html! {
            <article class={format!{"message is-light is-small is-{}", t}}>
            <div class="message-header">
                <p>{value.room.clone()}</p>
                <button class="delete" aria-label="delete" onclick = {ctx.link().callback(move |_| MessageItemMsg::Close(id))}></button>
            </div>
            <div class="message-body"  style={style} onclick = {onclick}>
               {content}
            </div>
            </article>
        }
    }
}
