#![allow(dead_code)]

use super::message_item::{MessageItem, MessageItemType, MessageItemValue};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew::Properties;

use super::message_dialog::MessageDialog;
use crate::util::request;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use std::collections::HashMap;
use std::collections::LinkedList;
use wasm_bindgen_futures::spawn_local;

const DEFAULT_ROOM: &str = "main";
const UPDATE_SESSION_PRE: &str = "update_session:";
const LIST_PRE: &str = "list:";
const JOIN_ROOM_PRE: &str = "join_room:";
const QUIT_ROOM_PRE: &str = "quit_room:";
const UPDATE_NAME_PRE: &str = "update_name:";
const MESSAGE_PRE: &str = "message:";
#[derive(Deserialize)]
struct UpdateName<'a> {
    pub session_id: &'a str,
    pub name: &'a str,
    pub old_name: &'a str,
}

#[derive(Deserialize, Debug)]
struct RoomChange<'a> {
    pub session_id: &'a str,
    pub name: &'a str,
    pub room: &'a str,
}

#[derive(Default)]
pub struct MessageList {
    ws_writer: Option<Arc<Mutex<SplitSink<WebSocket, Message>>>>,
    rooms: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    session_id: Arc<Mutex<Option<String>>>,
    messages: Arc<Mutex<HashMap<String, LinkedList<MessageContent>>>>,
    dialog_closed: Arc<Mutex<bool>>,
    latest_message: Option<MessageContent>,
}

#[derive(Clone, Properties)]
pub struct MessageListProps {
    #[prop_or_default]
    pub value: Arc<Mutex<MessageListValue>>,
    #[prop_or_default]
    pub ws: bool,
}

pub type MessageListValue = LinkedList<MessageItemValue>;

impl PartialEq for MessageListProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

pub enum MessageListMsg {
    Remove(u128),
    OpenDialog(u128),
    InitWS,
    ShowMsg(MessageContent),
    Notify,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MessageContent {
    pub id: u128,
    pub room: String,
    pub from_id: String,
    pub from_name: String,
    pub content: String,
    pub time: String,
    pub is_own: Option<()>,
}

pub trait MessageOperate {
    fn ok(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
    fn message(&self, room: &str, from_id: &str, from_name: &str, content: &str);
}

fn push(list: &Arc<Mutex<MessageListValue>>, item: MessageItemValue) {
    list.lock().unwrap().push_back(item);
}

fn remove(list: &Arc<Mutex<MessageListValue>>, id: u128) -> Option<MessageItemValue> {
    let index = list
        .lock()
        .unwrap()
        .iter()
        .enumerate()
        .find(|&(_, v)| v.id == id)
        .map(|(index, _)| index);
    if let Some(index) = index {
        return Some(list.lock().unwrap().remove(index));
    }
    None
}

impl MessageOperate for Arc<Mutex<MessageListValue>> {
    fn ok(&self, msg: &str) {
        push(
            self,
            MessageItemValue::new(
                MessageItemType::Success,
                "Success",
                msg,
                Some(5),
                None,
                None,
            ),
        )
    }
    fn warn(&self, msg: &str) {
        push(
            self,
            MessageItemValue::new(
                MessageItemType::Warning,
                "Warning",
                msg,
                Some(8),
                None,
                None,
            ),
        )
    }
    fn info(&self, msg: &str) {
        push(
            self,
            MessageItemValue::new(MessageItemType::Info, "Info", msg, Some(5), None, None),
        )
    }
    fn error(&self, msg: &str) {
        push(
            self,
            MessageItemValue::new(MessageItemType::Danger, "Error", msg, Some(10), None, None),
        )
    }

    fn message(&self, room: &str, from_id: &str, from_name: &str, content: &str) {
        push(
            self,
            MessageItemValue::new(
                MessageItemType::Primary,
                room,
                content,
                None,
                Some(from_name),
                Some(from_id),
            ),
        )
    }
}

impl Component for MessageList {
    type Message = MessageListMsg;

    type Properties = MessageListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ws_writer: None,
            session_id: Default::default(),
            rooms: Default::default(),
            messages: Default::default(),
            dialog_closed: Arc::new(Mutex::new(true)),
            latest_message: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessageListMsg::Remove(id) => match remove(&ctx.props().value, id) {
                Some(_) => true,
                _ => false,
            },
            MessageListMsg::OpenDialog(id) => {
                *self.dialog_closed.lock().unwrap() = false;
                let value = ctx.props().value.lock().unwrap();

                let item = value.iter().find(|x| x.id == id).unwrap().clone();

                self.latest_message = Some(MessageContent {
                    id: 0,
                    room: item.room,
                    from_id: item.from_id.unwrap_or_default(),
                    from_name: item.from.unwrap_or_default(),
                    content: item.content,
                    time: "".to_string(),
                    is_own: None,
                });

                for msg in value.iter() {
                    ctx.link().send_message(MessageListMsg::Remove(msg.id));
                }

                true
            }
            MessageListMsg::InitWS => {
                let ws = request::open_ws().unwrap();

                let (writer, mut reader) = ws.split();

                self.ws_writer = Some(Arc::new(Mutex::new(writer)));
                match self.ws_writer {
                    Some(ref w) => {
                        let w1 = Arc::clone(w);
                        spawn_local(async move {
                            w1.lock()
                                .unwrap()
                                .send(Message::Text(String::from("i am back online!")))
                                .await
                                .unwrap();
                        });
                    }
                    None => (),
                }

                let link = ctx.link().clone();
                let self_messages = self.messages.clone();
                let self_rooms = self.rooms.clone();
                let session_id = self.session_id.clone();
                let session_id_c = session_id.clone();
                let mut sid = session_id_c.lock().unwrap();
                if sid.is_none() {
                    let user = &crate::util::common::get_current_user().unwrap();
                    *sid = Some(user.name.clone().unwrap_or(user.email.clone()));
                }
                let sr1 = self_rooms.clone();
                let mut sr = sr1.lock().unwrap();
                if sr.is_empty() {
                    sr.insert(DEFAULT_ROOM.to_string(), HashMap::new());
                }
                let dialog_closed = self.dialog_closed.clone();
                spawn_local(async move {
                    while let Some(msg) = reader.next().await {
                        match msg {
                            Ok(msg) => {
                                // 【{room}】{name}: {msg}
                                if let Message::Text(content) = msg {
                                    if content.starts_with(MESSAGE_PRE) {
                                        let message: MessageContent = serde_json::from_str(
                                            content.trim_start_matches(MESSAGE_PRE),
                                        )
                                        .unwrap();
                                        self_messages
                                            .lock()
                                            .unwrap()
                                            .entry(message.room.clone())
                                            .or_insert(Default::default())
                                            .push_back(message.clone());
                                        if *dialog_closed.lock().unwrap() {
                                            link.send_message(MessageListMsg::ShowMsg(message));
                                        } else {
                                            link.send_message(MessageListMsg::Notify);
                                        }
                                    } else {
                                        if content.starts_with(LIST_PRE) {
                                            let rooms: HashMap<String, HashMap<String, String>> =
                                                serde_json::from_str(
                                                    content.trim_start_matches(LIST_PRE),
                                                )
                                                .unwrap();

                                            *self_rooms.lock().unwrap() = rooms;
                                        } else if content.starts_with(JOIN_ROOM_PRE) {
                                            let change: RoomChange = serde_json::from_str(
                                                content.trim_start_matches(JOIN_ROOM_PRE),
                                            )
                                            .unwrap();
                                            let mut sr = self_rooms.lock().unwrap();

                                            if change.session_id
                                                == session_id.lock().unwrap().as_deref().unwrap()
                                            {
                                                sr.entry(change.room.to_string())
                                                    .or_insert(HashMap::new());
                                            }

                                            if let Some(hm) = sr.get_mut(change.room) {
                                                hm.insert(
                                                    change.session_id.to_string(),
                                                    change.name.to_string(),
                                                );
                                            }
                                        } else if content.starts_with(QUIT_ROOM_PRE) {
                                            let change: RoomChange = serde_json::from_str(
                                                content.trim_start_matches(QUIT_ROOM_PRE),
                                            )
                                            .unwrap();

                                            let mut sr = self_rooms.lock().unwrap();

                                            if change.session_id
                                                == session_id.lock().unwrap().as_deref().unwrap()
                                            {
                                                sr.remove(change.room);
                                            } else {
                                                if let Some(hm) = sr.get_mut(change.room) {
                                                    hm.remove(change.session_id);
                                                }
                                            }
                                        } else if content.starts_with(UPDATE_NAME_PRE) {
                                            let change: UpdateName = serde_json::from_str(
                                                content.trim_start_matches(UPDATE_NAME_PRE),
                                            )
                                            .unwrap();

                                            for (_, sessions) in
                                                self_rooms.lock().unwrap().iter_mut()
                                            {
                                                sessions
                                                    .entry(change.session_id.to_string())
                                                    .and_modify(|x| *x = change.name.to_string());
                                            }
                                        }
                                        link.send_message(MessageListMsg::Notify);
                                    }
                                }
                            }
                            Err(err) => match err {
                                gloo_net::websocket::WebSocketError::ConnectionError => {
                                    log::error!("connection error: {:#?}", err);
                                    break;
                                }
                                gloo_net::websocket::WebSocketError::ConnectionClose(e) => {
                                    log::info!("connection closed, close event: {:#?}", e);
                                    break;
                                }
                                gloo_net::websocket::WebSocketError::MessageSendError(e) => {
                                    log::error!("message send error: {:#?}", e);
                                }
                                _ => {
                                    log::error!("read error: {:#?}", err);
                                }
                            },
                        }
                    }
                });

                false
            }
            MessageListMsg::ShowMsg(message) => {
                ctx.props().value.message(
                    &message.room,
                    &message.from_id,
                    &message.from_name,
                    &message.content,
                );
                true
            }
            MessageListMsg::Notify => true,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if ctx.props().ws && first_render {
            ctx.link().send_message(MessageListMsg::InitWS);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list = ctx.props().value.lock().unwrap();
        html! {
            <>
            <div class="message-list">
            {
                list.iter().map(|x|html!{
                    <MessageItem value = {x.clone()} on_close={ctx.link().callback(|id|{MessageListMsg::Remove(id)})} on_open_dialog={ctx.link().callback(|id|{MessageListMsg::OpenDialog(id)})}/>
                }).collect::<Html>()
            }
            </div>

            {
                if self.ws_writer.is_some() && !*self.dialog_closed.lock().unwrap(){
                    let session_id = self.session_id.lock().unwrap().clone().unwrap();
                    html!{
                        <MessageDialog session_id={session_id} rooms={self.rooms.clone()} messages={self.messages.clone()}  ws_writer = {self.ws_writer.clone()} closed = {self.dialog_closed.clone()} latest_message = {self.latest_message.clone()}/>
                    }
                }else{
                    html!{}
                }
            }
            </>
        }
    }
}
