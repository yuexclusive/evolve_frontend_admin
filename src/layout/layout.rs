use crate::component::menu::{Menu, MenuLabel};
use crate::layout::header::Header;
use crate::util::common;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(PartialEq, Properties)]
pub struct BodyProps {
    #[prop_or_default]
    pub content: VNode,
    #[prop_or_default]
    pub menus: Vec<MenuLabel>,
}

pub struct Item<'a> {
    pub path: &'a str,
    pub navbar_name: Option<&'a str>,
    pub navbar_parent_name: Option<&'a str>,
    pub left_menu_name: Option<&'a str>,
}

pub fn gen_items<'a>() -> Vec<Item<'a>> {
    vec![
        Item {
            path: "/main/user",
            navbar_name: Some("User"),
            navbar_parent_name: Some("Modules"),
            left_menu_name: Some("User"),
        },
        Item {
            path: "/main/role",
            navbar_name: Some("User"),
            navbar_parent_name: Some("Modules"),
            left_menu_name: Some("Role"),
        },
        Item {
            path: "/",
            navbar_name: Some("Welcome"),
            navbar_parent_name: Some("Modules"),
            left_menu_name: None,
        },
    ]
}

pub fn get_selected_navbar_and_menu() -> (Option<String>, Option<String>, Option<String>) {
    let mut res = (None, None, None);
    let path = web_sys::window().unwrap().location().pathname().unwrap();
    let items = gen_items();
    let items = items.iter().find(|x| x.path == &path);
    if let Some(v) = items {
        res.0 = v.navbar_name.map(|x| x.to_string());
        res.1 = v.navbar_parent_name.map(|x| x.to_string());
        res.2 = v.left_menu_name.map(|x| x.to_string());
    }

    res
}

#[function_component(Layout)]
pub fn body(props: &BodyProps) -> Html {
    let (selected_navbar_name, selected_navbar_parent_name, selected_name) =
        get_selected_navbar_and_menu();
    let labels = props.menus.clone();
    let on_select_menu = Callback::from(move |name: String| {
        if let Some(item) = gen_items()
            .iter()
            .find(|x| x.left_menu_name.is_some_and(|x| x == &name))
        {
            common::redirect(item.path);
        }
    });
    html! {
        <>
             <Header selected_navbar_name={selected_navbar_name} selected_navbar_parent_name={selected_navbar_parent_name} />
             {
                if !props.menus.is_empty() {
                    html!{
                        <div class="columns is-gapless">
                            <div class="column is-2 left-container">
                                <Menu on_select={on_select_menu} selected_name = {selected_name} labels = { labels }/>
                            </div>
                            <div class="column is-10">
                                { props.content.clone() }
                            </div>
                        </div>
                    }
                } else{
                    props.content.clone()
                }
             }
        </>
    }
}
