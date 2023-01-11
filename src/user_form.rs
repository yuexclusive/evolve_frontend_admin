use std::cell::RefCell;
use std::rc::Rc;

// use crate::component::message_item::MessageItemValue;
use crate::component::message_list::{MessageList, MessageListValue, MessageOperate};
use crate::user_list::User;
use crate::util::request;
use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew::Properties;

pub struct UserForm {
    messages: Arc<Mutex<MessageListValue>>,
}

pub enum UserFormMsg {
    Close,
    NameChange(web_sys::Event),
    MobileChange(web_sys::Event),
    Update,
    UpdateSuccess,
    UpdateError(Box<dyn std::error::Error>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserFormProps {
    #[prop_or_default]
    pub value: RefCell<User>,
    #[prop_or_default]
    pub update: Callback<()>,
    pub closed: Rc<RefCell<bool>>,
}

impl Component for UserForm {
    type Message = UserFormMsg;

    type Properties = UserFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            messages: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserFormMsg::Close => {
                *ctx.props().closed.borrow_mut() = true;
                true
            }
            UserFormMsg::NameChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                ctx.props().value.borrow_mut().name = Some(el.value());
                false
            }
            UserFormMsg::MobileChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                ctx.props().value.borrow_mut().mobile = Some(el.value());
                false
            }
            UserFormMsg::Update => {
                let user = ctx.props().value.borrow().clone();
                ctx.link().send_future(async move {
                    match request::put::<User, _>(request::Host::ApiBase, "/user/update", &user).await
                    {
                        Ok(_) => UserFormMsg::UpdateSuccess,
                        Err(err) => UserFormMsg::UpdateError(Box::new(err)),
                    }
                });
                false
            }
            UserFormMsg::UpdateSuccess => {
                ctx.props().update.emit(());
                ctx.link().send_message(UserFormMsg::Close);
                false
            }
            UserFormMsg::UpdateError(_err) => {
                self.messages.error(&format!("{}", _err));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().closed.borrow().clone() {
            return html! {};
        }
        let val = ctx.props().value.borrow();
        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <MessageList value={self.messages.clone()}/>
                    <header class="modal-card-head">
                    <p class="modal-card-title">{"User Edit"}</p>
                    <button class="delete" aria-label="close" onclick={ctx.link().callback(|_|UserFormMsg::Close)}></button>
                    </header>
                    <section class="modal-card-body">

                    <fieldset disabled={true}>
                    <div class="field">
                        <label class="label">{"Type"}</label>
                        <div class="control">
                        <input class="input" value={val.r#type.clone()} type="text" />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{"Email"}</label>
                        <div class="control">
                        <input class="input" value={val.email.clone()} type="email" />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{"Status"}</label>
                        <div class="control">
                        <input class="input" value={val.status.clone()} />
                        </div>
                    </div>
                    </fieldset>



                    <div class="field">
                        <label class="label">{"Name"}</label>
                        <div class="control">
                        <input class="input" value={val.name.clone()} type="text" placeholder="Scarlett" onchange={ctx.link().callback(|e:web_sys::Event|UserFormMsg::NameChange(e))}/>
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{"Mobile"}</label>
                        <div class="control">
                        <input class="input" value={val.mobile.clone()} type="text" placeholder="13800001111" onchange={ctx.link().callback(|e:web_sys::Event|UserFormMsg::MobileChange(e))}/>
                        </div>
                    </div>

                    </section>
                    <footer class="modal-card-foot">
                    <button class="button is-success"  onclick={ctx.link().callback(|_|UserFormMsg::Update)}>{"Save changes"}</button>
                    <button class="button" onclick={ctx.link().callback(|_|UserFormMsg::Close)} >{"Cancel"}</button>
                    </footer>
                </div>
            </div>
        }
    }
}
