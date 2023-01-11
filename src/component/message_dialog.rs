// use crate::component::message_item::MessageItemValue;
use crate::component::menu::{Menu, MenuLabel, MenuNode};
use crate::component::message_list::MessageContent;
use futures::stream::SplitSink;
use futures::SinkExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
// use utilities::datetime::FormatDateTime;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;

pub struct MessageDialog {
    refs: Vec<NodeRef>,
    selected_room: Option<String>,
}

pub enum MessageDialogMsg {
    Close,
    Send(web_sys::KeyboardEvent),
    ClickRoom(String),
    Notify,
}

#[derive(Clone, Properties)]
pub struct MessageDialogProps {
    #[prop_or_default]
    pub closed: Arc<Mutex<bool>>,

    #[prop_or_default]
    pub rooms: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,

    #[prop_or_default]
    pub messages: Arc<Mutex<HashMap<String, LinkedList<MessageContent>>>>,

    #[prop_or_default]
    pub ws_writer: Option<Arc<Mutex<SplitSink<WebSocket, Message>>>>,

    #[prop_or_default]
    pub latest_message: Option<MessageContent>,
}

impl PartialEq for MessageDialogProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Component for MessageDialog {
    type Message = MessageDialogMsg;

    type Properties = MessageDialogProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut res = Self {
            refs: vec![NodeRef::default(), NodeRef::default()],
            selected_room: None,
        };
        if let Some(v) = &ctx.props().latest_message {
            res.selected_room = Some(v.room.clone());
        }
        res
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessageDialogMsg::Close => {
                *ctx.props().closed.lock().unwrap() = true;
                true
            }
            MessageDialogMsg::Send(e) => {
                if e.key_code() == 13 {
                    let email_input = &self.refs[1];
                    let input = email_input.cast::<HtmlInputElement>().unwrap();
                    let content = input.value();
                    let messages = ctx.props().messages.clone();
                    if !e.meta_key() {
                        e.prevent_default();
                        if let Some(room) = self.selected_room.as_deref() {
                            if let Some(ws_writer) = &ctx.props().ws_writer {
                                let w1 = ws_writer.clone();
                                let room = room.to_string();
                                let user = crate::util::common::get_current_user().unwrap();
                                let name = user.name.unwrap_or(user.email.clone());
                                let link = ctx.link().clone();
                                spawn_local(async move {
                                    w1.lock()
                                        .unwrap()
                                        .send(Message::Text(content.clone()))
                                        .await
                                        .unwrap();
                                    input.set_value("");
                                    messages.lock().unwrap().entry(room.clone()).and_modify(
                                        |linked_list| {
                                            linked_list.push_back(MessageContent {
                                                id: 0,
                                                room: room.to_string(),
                                                from_id: "".to_string(),
                                                from_name: name,
                                                content: content.clone(),
                                                time: "".to_string(), //chrono::Utc::now().to_default(),
                                                is_own: Some(()),
                                            });
                                        },
                                    );
                                    link.send_message(MessageDialogMsg::Notify);
                                });
                            }
                        }
                    } else {
                        input.set_value(&(input.value() + "\n"));
                    }
                }
                false
            }
            MessageDialogMsg::ClickRoom(name) => {
                self.selected_room = Some(name);
                true
            }
            MessageDialogMsg::Notify => true,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let message_input = &self.refs[0];
        message_input
            .cast::<HtmlInputElement>()
            .and_then::<(), _>(|input| {
                let msg = match self.selected_room.as_deref() {
                    Some(room) => ctx
                        .props()
                        .messages
                        .lock()
                        .unwrap()
                        .get(room)
                        .unwrap_or(&Default::default())
                        .iter()
                        .map(|x| format!("{}: {}\n\n", x.from_name, x.content))
                        .collect::<String>(),
                    None => "".to_string(),
                };
                input.set_value(&msg);
                input.set_scroll_top(input.scroll_height());
                None
            });
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if *ctx.props().closed.lock().unwrap() {
            return html! {};
        }
        let room_nodes = ctx
            .props()
            .rooms
            .lock()
            .unwrap()
            .iter()
            .map(|(room, _)| MenuNode {
                name: room.to_string(),
                children: vec![],
            })
            .collect::<Vec<MenuNode>>();

        let room_labels = vec![MenuLabel {
            label: None,
            nodes: room_nodes,
        }];

        let mut session_nodes = vec![];

        if let Some(room) = self.selected_room.as_deref() {
            if let Some(sessions) = ctx.props().rooms.lock().unwrap().get(room) {
                for (_, name) in sessions.iter() {
                    session_nodes.push(MenuNode {
                        name: name.to_string(),
                        children: vec![],
                    });
                }
            }
        }

        let session_labels = vec![MenuLabel {
            label: None,
            nodes: session_nodes,
        }];

        let title = self.selected_room.as_deref().unwrap_or("Dialog");

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card" style="height:70%;width:60%;">
                    <header class="modal-card-head">
                    <p class="modal-card-title">{title}</p>
                    <button class="delete" aria-label="close" onclick={ctx.link().callback(|_|MessageDialogMsg::Close)}></button>
                    </header>

                    <section class="modal-card-body">
                    <div class="columns" style="height:100%;">
                    <div class="column is-2">
                        <div style="height: 100%; overflow: scroll;">
                            <Menu labels = {room_labels} selected_name = {self.selected_room.clone()} on_select = {ctx.link().callback(|name|MessageDialogMsg::ClickRoom(name))}/>
                        </div>
                    </div>
                    <div class="column is-7">
                        <div style="height: 70%;">
                            <textarea ref={&self.refs[0]} style="height: 100%;" readonly={true} class="textarea has-fixed-size"></textarea>
                        </div>
                        <div style="margin-top: 0.8em;">
                            <textarea ref={&self.refs[1]} class="textarea has-fixed-size" onkeydown={ctx.link().callback(|e:web_sys::KeyboardEvent|MessageDialogMsg::Send(e))} />
                        </div>
                    </div>
                    <div class="column is-3">
                        <div style="height: 100%; overflow: scroll;">
                            <Menu labels = {session_labels}/>
                        </div>
                    </div>
                    </div>
                    </section>
                </div>
            </div>
        }
    }
}
