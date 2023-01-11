use crate::user_list::{SearchedUser, User};
use std::ops::Deref;
use yew::prelude::*;
use yew::Properties;
use crate::util::common;

pub struct UserListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct UserListItemProps {
    pub value: SearchedUser,
    #[prop_or_default]
    pub is_selected: bool,
    pub onselect: Callback<User>,
}

pub enum UserListItemMsg {
    Select,
}

impl Component for UserListItem {
    type Message = UserListItemMsg;
    type Properties = UserListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListItemMsg::Select => {
                let val = ctx.props().value.clone();
                ctx.props().onselect.emit(val.user);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let user = &ctx.props().value.user;
        let formatter = &ctx.props().value.formatter;
        let is_selected = ctx.props().is_selected;

        html! {
            <tr class = {if is_selected {"is-selected"} else {""}}
             onclick = {ctx.link().callback(|_| UserListItemMsg::Select)} >
                {common::create_html("td",formatter.r#type.deref())}
                {common::create_html("td",formatter.email.deref())}
                {common::create_html("td",formatter.name.as_deref().unwrap_or(""))}
                {common::create_html("td",formatter.mobile.as_deref().unwrap_or(""))}
                {common::create_html("td",formatter.laston.as_deref().unwrap_or(""))}
                {common::create_html("td",formatter.created_at.deref())}
                {common::create_html("td",formatter.updated_at.as_deref().unwrap_or(""))}
                {common::create_html("td",formatter.status.deref())}
            </tr>
        }
    }
}
