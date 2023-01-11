use crate::component::message_list::{MessageList, MessageListValue, MessageOperate};
use crate::component::pager::{Page, Pager};
use crate::confirm_form::ConfirmForm;
use crate::user_form::UserForm;
use crate::user_list_item::UserListItem;
use crate::util::request;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew::Properties;

pub struct UserList {
    selected_row: Option<User>,
    user_form_closed: Rc<RefCell<bool>>,
    confirm_form_closed: Rc<RefCell<bool>>,
    messages: Arc<Mutex<MessageListValue>>,
    loading: bool,
    key_word: Option<String>,
}

pub enum UserListMsg {
    Refresh,
    HandleSearchSuccess(Vec<SearchedUser>, usize),
    HandleSearchFail(Box<dyn std::error::Error>),
    PageChanged(Page),
    OnSelect(User),
    Edit,
    Delete,
    DeleteConfirm,
    HandleDeleteSuccess,
    HandleDeleteFail(Box<dyn std::error::Error>),
    KeywordChange(web_sys::KeyboardEvent),
}
// #[derive(Serialize, Component)]
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
pub struct User {
    pub id: i64,
    pub r#type: String,
    pub email: String,
    pub status: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
pub struct UserFormatter {
    pub r#type: String,
    pub email: String,
    pub status: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Default)]
pub struct SearchedUser {
    pub user: User,
    pub formatter: UserFormatter,
}

#[derive(Serialize)]
pub struct DeleteReq {
    pub ids: Vec<i64>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserListProps {
    #[prop_or_default]
    pub data: RefCell<Vec<SearchedUser>>,

    #[prop_or_default]
    pub page: RefCell<Page>,
}

impl Component for UserList {
    type Message = UserListMsg;

    type Properties = UserListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_row: Default::default(),
            user_form_closed: Rc::new(RefCell::new(true)),
            confirm_form_closed: Rc::new(RefCell::new(true)),
            messages: Default::default(),
            loading: Default::default(),
            key_word: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListMsg::Refresh => {
                self.selected_row = None;
                self.loading = true;
                let page = ctx.props().page.borrow();
                let params = vec![
                    ("key_word", self.key_word.clone().unwrap_or_default()),
                    ("index", page.index.to_string()),
                    ("size", page.size.to_string()),
                ];

                ctx.link().send_future(async move {
                    match request::get::<Vec<SearchedUser>, _, _>(
                        request::Host::ApiBase,
                        "/user/search",
                        Some(params),
                    )
                    .await
                    {
                        Ok(res) => {
                            UserListMsg::HandleSearchSuccess(res.data.unwrap(), res.total.unwrap())
                        }
                        Err(err) => UserListMsg::HandleSearchFail(Box::new(err)),
                    }
                });
                false
            }
            UserListMsg::HandleSearchFail(_err) => {
                self.messages.error(&format!("{}", _err));
                self.loading = false;
                true
            }
            UserListMsg::HandleSearchSuccess(v, total) => {
                *ctx.props().data.borrow_mut() = v;
                ctx.props().page.borrow_mut().total = total;
                self.loading = false;
                true
            }
            UserListMsg::PageChanged(page) => {
                *ctx.props().page.borrow_mut() = page;
                ctx.link().send_message(UserListMsg::Refresh);
                true
            }
            UserListMsg::OnSelect(user) => {
                self.selected_row = Some(user);
                true
            }
            UserListMsg::Edit => {
                if self.selected_row.is_none() {
                    log::info!("{:#?}", self.messages);
                    self.messages.warn("please select a record");
                } else {
                    *self.user_form_closed.borrow_mut() = false;
                }
                true
            }
            UserListMsg::Delete => {
                if self.selected_row.is_none() {
                    self.messages.warn("please select a record");
                } else {
                    *self.confirm_form_closed.borrow_mut() = false;
                }
                true
            }
            UserListMsg::DeleteConfirm => {
                let body = DeleteReq {
                    ids: Vec::from([self.selected_row.clone().unwrap().id]),
                };
                ctx.link().send_future(async move {
                    match request::delete::<u64, _>(request::Host::ApiBase, "/user/delete", &body)
                        .await
                    {
                        Ok(_) => UserListMsg::HandleDeleteSuccess,
                        Err(err) => UserListMsg::HandleDeleteFail(Box::new(err)),
                    }
                });
                true
            }
            UserListMsg::HandleDeleteSuccess => {
                *self.confirm_form_closed.borrow_mut() = true;
                ctx.link().send_message(UserListMsg::Refresh);
                false
            }
            UserListMsg::HandleDeleteFail(_err) => {
                self.messages.error(&format!("{}", _err));
                true
            }
            UserListMsg::KeywordChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                self.key_word = Some(el.value());
                ctx.props().page.borrow_mut().to_start();
                ctx.link().send_message(UserListMsg::Refresh);
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(UserListMsg::Refresh);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let data = ctx.props().data.borrow().clone();
        let selected_id = self.selected_row.clone().map(|x| x.id);
        html! {
        <>
        <MessageList value = {self.messages.clone()} ws = false/>
        if let Some(v) = &self.selected_row  {
            <UserForm value = {RefCell::new(v.clone())} closed={self.user_form_closed.clone()} update = {ctx.link().callback(|_|{UserListMsg::Refresh})}/>
            <ConfirmForm closed={self.confirm_form_closed.clone()} confirm = {ctx.link().callback(|_|{UserListMsg::DeleteConfirm})} content = {"Deleted users <b>can not</b> be recovered!!!<br/> are you sure you want to delete it?"}/>
        }
        <div class="search-container">
            <div class="search-input field is-grouped">
            <p class="control is-expanded">
                <input class="input" type="text" onkeyup={ctx.link().callback(|e:web_sys::KeyboardEvent|UserListMsg::KeywordChange(e))} placeholder="Search"/>
            </p>

            <p class="control">
                <button class="button is-light is-warning" onclick={ctx.link().callback(|_|UserListMsg::Edit)}>{"Edit"}</button>
            </p>
            <p class="control">
                <button class="button is-light is-danger" onclick={ctx.link().callback(|_|UserListMsg::Delete)}>{"Delete"}</button>
            </p>
            </div>
        </div>
        <div class="table-container">
            {
                if self.loading {
                    html!{
                        <div class="table-loading">
                        </div>
                    }
                }else{
                    html!{}
                }
            }

            <table class="table is-bordered is-striped is-narrow is-hoverable">
            <thead>
                <tr>
                <th><abbr title="Type">{"Type"}</abbr></th>
                <th><abbr title="Email">{"Email"}</abbr></th>
                <th><abbr title="Name">{"Name"}</abbr></th>
                <th><abbr title="Mobile">{"Mobile"}</abbr></th>
                <th><abbr title="Laston">{"Laston"}</abbr></th>
                // todo: sort
                <th><abbr title="Created_at"><a href="javascript:void(0)">{"Created_at  "}<i class="fa-solid fa-arrow-down"></i></a></abbr></th>
                <th><abbr title="Updated_at">{"Updated_at"}</abbr></th>
                <th><abbr title="Status">{"Status"}</abbr></th>
                </tr>
            </thead>
            <tbody>
            {
                data.iter().map(|x| html!{
                    <UserListItem is_selected = {selected_id.is_some() && selected_id.clone().unwrap()==x.user.id} onselect={ctx.link().callback(|user|{UserListMsg::OnSelect(user)})}  value={x.clone()} />
                }).collect::<Html>()
            }
            </tbody>
            </table>
        </div>
        <div class="pager-container">
        {
            html!{
                <Pager value = { RefCell::new(ctx.props().page.borrow().clone())} page_changed = {ctx.link().callback(|page|UserListMsg::PageChanged(page))}/>
            }
        }
        </div>

        </>
                        }
    }
}
