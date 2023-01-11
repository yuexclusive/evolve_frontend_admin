use crate::util::common;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub enum NavbarMsg {
    OnSelect(String),
}

pub struct Navbar {
    data: Vec<NavbarNode>,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    #[prop_or_default]
    pub selected_navbar_name: Option<String>,
    #[prop_or_default]
    pub selected_navbar_parent_name: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NavbarNode {
    name: String,
    children: Vec<NavbarNode>,
    path: Option<String>,
    divider: bool,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = NavbarProps;

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavbarMsg::OnSelect(path) => {
                common::redirect(&path);
                true
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        let data = vec![NavbarNode {
            name: "Modules".to_string(),
            path: None,
            divider: false,
            children: vec![
                NavbarNode {
                    name: "Welcome".to_string(),
                    path: Some("/".to_string()),
                    divider: true,
                    children: vec![],
                },
                NavbarNode {
                    name: "User".to_string(),
                    path: Some("/main/user".to_string()),
                    divider: false,
                    children: vec![],
                },
            ],
        }];

        Self { data }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected_navbar_name = ctx.props().selected_navbar_name.clone();
        let selected_navbar_parent_name = ctx.props().selected_navbar_parent_name.clone();
        html! {
            <div class="navbar-start">
            {
                self.data
                .iter()
                .map(|item| {
                    let path = item.path.clone();
                    if item.children.is_empty() {
                        html! {
                            <a href={String::from("javascript:void(0)")} onclick = {ctx.link().callback(move|_|NavbarMsg::OnSelect(path.clone().unwrap()))} class={if selected_navbar_name.is_some() && &item.name == &selected_navbar_name.clone().unwrap()  {"navbar-item is-active"} else {"navbar-item"}}>
                                {item.name.clone()}
                            </a>
                        }
                    }else{
                        html!{
                        <div class="navbar-item has-dropdown is-hoverable">
                                <a href={String::from("javascript:void(0)")} class={if selected_navbar_parent_name.is_some() && &item.name == &selected_navbar_parent_name.clone().unwrap() {"navbar-link is-active"} else {"navbar-link"}}>
                                    {item.name.clone()}
                                </a>
                                <div class="navbar-dropdown">
                                    {
                                        item.children.iter().map(|child_item|{
                                            let class = {if selected_navbar_name.is_some() && &child_item.name == &selected_navbar_name.clone().unwrap() {"navbar-item is-active"} else {"navbar-item"}};
                                            let path = child_item.path.clone();
                                            html!{
                                                <>
                                                <a href={String::from("javascript:void(0)")} onclick = {ctx.link().callback(move|_|NavbarMsg::OnSelect(path.clone().unwrap()))} class={class} >
                                                    {child_item.name.clone()}
                                                </a>
                                                {
                                                    if child_item.divider{
                                                    html!{ <hr class="navbar-divider"/> }
                                                    }else{
                                                    html!{}
                                                    }
                                                }
                                                </>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                        </div>
                        }
                    }
                })
                .collect::<Html>()
            }
            </div>
        }
    }
}
