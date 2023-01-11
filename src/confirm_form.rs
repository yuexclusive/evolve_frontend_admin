use crate::util::common;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct ConfirmFormProps {
    pub confirm: Callback<()>,
    pub closed: Rc<RefCell<bool>>,
    pub content: String,
    #[prop_or(true)]
    pub show_cancel: bool,
    #[prop_or(true)]
    pub show_close: bool,
}

#[function_component(ConfirmForm)]
pub fn confirm_form(props: &ConfirmFormProps) -> Html {
    let re_render = use_state(|| true);
    let close = {
        let a = props.closed.clone();
        Callback::from(move |_| {
            *a.borrow_mut() = true;
            re_render.set(true);
        })
    };

    let confirm = {
        let props = props.clone();
        Callback::from(move |_| {
            props.confirm.emit(());
        })
    };

    if *props.closed.borrow() {
        return html! {};
    }

    html! {
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                <p class="modal-card-title">{"Confirm"}</p>
                <button class="delete" aria-label="close" onclick={close.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {
                        common::create_html("div",&props.content)
                    }
                </section>
                <footer class="modal-card-foot">
                <button class="button is-danger"  onclick={confirm}>{"Confirm"}</button>
                <button class="button" onclick={close}>{"Cancel"}</button>
                </footer>
            </div>
        </div>
    }
}
