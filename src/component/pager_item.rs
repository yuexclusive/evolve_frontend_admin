use yew::prelude::*;
use yew::Properties;

pub struct PagerItem;

#[derive(Clone, PartialEq, Properties)]
pub struct UserListPageItemProps {
    pub onclick: Callback<usize>,
    pub page_index: usize,
    pub active: bool,
}

pub enum UserListPageItemMsg {
    Click,
}

impl Component for PagerItem {
    type Message = UserListPageItemMsg;

    type Properties = UserListPageItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListPageItemMsg::Click => {
                ctx.props().onclick.emit(ctx.props().page_index);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let index = ctx.props().page_index;
        let active = ctx.props().active;
        let onclick = ctx.link().callback(|_| UserListPageItemMsg::Click);
        html! {
            <>
                <li><a href={format!("javascript:void(0)")} {onclick} class={if active {"pagination-link is-current"} else { "pagination-link" }}>{index}</a></li>
            </>
        }
    }
}
