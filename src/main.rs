#![feature(is_some_and)]
#![feature(linked_list_remove)]
mod component;
mod confirm_form;
mod error_page;
mod forget_pwd;
mod layout;
mod login;
mod register;
mod role_list;
mod user_form;
mod user_list;
mod user_list_item;
mod util;

use component::menu::{MenuLabel, MenuNode};
use component::welcome::Welcome;
use error_page::{page_not_found::PageNotFound, request_error::RequestError};
use forget_pwd::ForgetPwd;
use layout::layout::Layout;
use login::Login;
use register::Register;
use role_list::RoleList;
use user_list::UserList;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;
use yew_router::BrowserRouter;

#[derive(Clone, Routable, PartialEq)]
enum RouteBody {
    #[not_found]
    #[at("/page_not_found")]
    PageNotFound,
    #[at("/main/user")]
    User, // Hello,
    #[at("/main/role")]
    Role, // Hello,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/page_not_found")]
    PageNotFound,
    #[at("/")]
    Welcome,
    #[at("/login")]
    Login,
    #[at("/forget_pwd")]
    ForgetPwd,
    #[at("/register")]
    Register,
    #[at("/main/:?")]
    Body,
    #[at("/401")]
    Unauthorized,
    #[at("/404")]
    NotFound,
}

fn switch_body(route: RouteBody) -> VNode {
    match route {
        RouteBody::PageNotFound => {
            html! {
                <PageNotFound />
            }
        }
        RouteBody::User => {
            html! {
                <UserList />
            }
        }

        RouteBody::Role => {
            html! {
                <RoleList />
            }
        }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => {
            html! {
                <Login />
            }
        }
        Route::Register => {
            html! {
                <Register />
            }
        }
        Route::ForgetPwd => {
            html! {
                <ForgetPwd />
            }
        }
        Route::Welcome => {
            html! {
                <Layout content={html!{<Welcome />}}/>
            }
        }
        Route::Body => {
            let menus = vec![MenuLabel {
                label: Some(String::from("User Management")),
                nodes: vec![
                    MenuNode {
                        name: String::from("User"),
                        children: Default::default(),
                    },
                    MenuNode {
                        name: String::from("Role"),
                        children: Default::default(),
                    },
                ],
            }];
            html! {
                <Layout menus = {menus} content={html!{<Switch<RouteBody> render={switch_body} />}}/>
            }
        }
        Route::Unauthorized => {
            html! {
                <RequestError status={401} />
            }
        }

        Route::NotFound => {
            html! {
                <RequestError status={404} />
            }
        }

        Route::PageNotFound => {
            html! {
                <PageNotFound />
            }
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
