use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::Properties;

pub enum MenuMsg {
    OnSelect(String),
}

pub struct Menu;

#[derive(Clone, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub labels: Vec<MenuLabel>,
    #[prop_or_default]
    pub on_select: Callback<String>,
    #[prop_or_default]
    pub selected_name: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MenuLabel {
    pub label: Option<String>,
    pub nodes: Vec<MenuNode>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MenuNode {
    pub name: String,
    pub children: Vec<MenuNode>,
}

impl MenuNode {
    fn render(&self, menu: &Menu, ctx: &Context<Menu>) -> Html {
        let name = self.name.clone();
        let mut class = "";
        if let Some(v) = ctx.props().selected_name.as_deref() {
            if v == &name {
                class = "is-active"
            }
        }
        html! {
            <li>
                <a href={String::from("javascript:void(0)")} class={class} onclick = {ctx.link().callback(move|_|MenuMsg::OnSelect(name.clone()))}>{&self.name}</a>
                {
                    if self.children.is_empty() {
                        html!{}
                    } else{
                        html!{
                            <ul>
                                {
                                    self.children.iter().map(|n|n.render(menu,ctx)).collect::<Html>()
                                }
                            </ul>
                        }
                    }
                }
            </li>
        }
    }
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = MenuProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MenuMsg::OnSelect(name) => {
                ctx.props().on_select.emit(name.clone());
                true
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                    <aside class="menu">
                    {
                        ctx.props().labels.iter().map(|x|html!{
        <>
        {
            if let Some(label) = &x.label{
                html!{
                    <p class="menu-label">
                    {label}
                    </p>
                }
            } else{
                html!{}
            }
        }
        <ul class="menu-list">
            {
                x.nodes.iter().map(|n|html!{
                    n.render(self,ctx)
                }).collect::<Html>()
            }
        </ul>
        </>
                        }).collect::<Html>()
                    }
                    </aside>
                }
    }
}
