use crate::layout::navbar::Navbar;
use crate::util::common;
use yew::prelude::*;

pub struct Header {
    navbar_active: bool,
    current_user: Option<common::CurrentUser>,
}

pub enum HeaderMsg {
    Logout,
    ToggleNavbarActive,
}
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub selected_navbar_name: Option<String>,
    #[prop_or_default]
    pub selected_navbar_parent_name: Option<String>,
}

impl Component for Header {
    type Message = HeaderMsg;

    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut res = Self {
            navbar_active: false,
            current_user: Default::default(),
        };

        match common::get_current_user() {
            Ok(v) => res.current_user = Some(v),
            Err(err) => {
                log::warn!("get current user error: {}", err);
                // common::redirect("/401");
                common::redirect("/login");
            }
        }
        res
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderMsg::Logout => {
                common::delete_current_user().unwrap_or_else(|x| {
                    log::error!("{:?}", x);
                });
                common::redirect("/login");
                true
            }
            HeaderMsg::ToggleNavbarActive => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = self.current_user.clone().unwrap();
        let navbar_active = if self.navbar_active { "is-active" } else { "" };
        html! {
            <div class="header-container">
                <nav class="navbar is-light" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <a class="navbar-item" href="/">
                            <img alt="fuck you" src="/static/img/logo.png" width="100" height="100"/>
                        </a>

                        <a href={String::from("javascript:void(0)")} role="button" onclick={ctx.link().callback(|_|HeaderMsg::ToggleNavbarActive)} class={format!{"navbar-burger {navbar_active}"}} aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        </a>
                    </div>

                    <div id="navbarBasicExample" class={format!("navbar-menu {navbar_active}")}>
                        <Navbar selected_navbar_name={ctx.props().selected_navbar_name.clone()} selected_navbar_parent_name={ctx.props().selected_navbar_parent_name.clone()}/>
                        <div class="navbar-end">
                            <div class="navbar-item has-dropdown is-hoverable">
                                <a href={String::from("javascript:void(0)")} class="navbar-link" style="color:#000000">
                                    { user.name.unwrap_or("unnamed".to_string())}
                                </a>

                                <div class="navbar-dropdown is-right">
                                <a href={String::from("javascript:void(0)")} class="navbar-item">
                                    {user.r#type}
                                </a>
                                <a href={String::from("javascript:void(0)")} class="navbar-item">
                                    {user.email}
                                </a>
                                <hr class="navbar-divider"/>
                                <a href={String::from("javascript:void(0)")} onclick={ctx.link().callback(|_|HeaderMsg::Logout)} class="navbar-item">
                                    {"Logout"}
                                </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}
