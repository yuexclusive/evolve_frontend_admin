use std::cell::RefCell;

use super::pager_item::PagerItem;
use serde::Deserialize;
use yew::prelude::*;
use yew::Properties;

pub struct Pager;

pub enum PagerMsg {
    PageItemClick(usize),
    Next,
    Pre,
    First,
    Last,
    SizeChanged(web_sys::Event),
}

const SPAN: usize = 8;

pub const DEFAULT_PAGE_SIZE: usize = 18;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Page {
    pub total: usize,
    pub index: usize,
    pub size: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            total: 0,
            index: 1,
            size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl Page {
    pub fn page_total(&self) -> usize {
        (self.total.max(1) - 1) / self.size + 1
    }

    pub fn page_start(&self) -> usize {
        ((self.index - 1) / SPAN) * SPAN + 1
    }

    pub fn page_end(&self) -> usize {
        (self.page_start() + SPAN - 1).min(self.page_total())
    }

    pub fn to_start(&mut self) {
        self.index = 1;
    }

    pub fn to_end(&mut self) {
        self.index = self.page_total();
    }

    pub fn to(&mut self, index: usize) {
        self.index = index;
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn pre(&mut self) {
        self.index -= 1;
    }

    pub fn is_start(&self) -> bool {
        self.index == 1
    }

    pub fn is_end(&self) -> bool {
        self.index == self.page_total()
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.index == index
    }

    pub fn change_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PagerProps {
    pub value: RefCell<Page>,
    pub page_changed: Callback<Page>,
}
impl Component for Pager {
    type Message = PagerMsg;

    type Properties = PagerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PagerMsg::PageItemClick(index) => {
                ctx.props().value.borrow_mut().to(index);
            }
            PagerMsg::Next => {
                ctx.props().value.borrow_mut().next();
            }
            PagerMsg::Pre => {
                ctx.props().value.borrow_mut().pre();
            }
            PagerMsg::First => {
                ctx.props().value.borrow_mut().to_start();
            }
            PagerMsg::Last => {
                ctx.props().value.borrow_mut().to_end();
            }
            PagerMsg::SizeChanged(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                ctx.props()
                    .value
                    .borrow_mut()
                    .change_size(el.value().parse().unwrap());
                ctx.props().value.borrow_mut().to_start();
            }
        }
        let page = ctx.props().value.borrow().clone();
        ctx.props().page_changed.emit(page);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = ctx.props().value.borrow();
        // if page.is_empty() {
        //     return html! {};
        // }
        let pre_class = if page.is_start() {
            "pagination-link is-disabled"
        } else {
            "pagination-link"
        };

        let next_class = if page.is_end() {
            "pagination-link is-disabled"
        } else {
            "pagination-link"
        };

        let pre_style = if page.is_start() {
            "pointer-events: none;"
        } else {
            ""
        };
        let next_style = if page.is_end() {
            "pointer-events: none;"
        } else {
            ""
        };
        html! {
            <nav class="pagination is-small is-rounded is-right ">
            <ul class="pagination-list">
                <li>
                    <div class="select is-small is-rounded">
                        <select onchange={ctx.link().callback(|e:web_sys::Event|PagerMsg::SizeChanged(e))}>
                            {
                                (0..=3).map(|i|{
                                    let size = DEFAULT_PAGE_SIZE<<i;
                                    html!{
                                        <option selected={page.size==size} value={size.to_string()}>{format!("size: {}",size)}</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>
                </li>
                <li><a href={format!("javascript:void(0)")} class={pre_class} style={pre_style} onclick = {ctx.link().callback(|_|PagerMsg::First)}>{"<<"}</a></li>
                <li><a href={format!("javascript:void(0)")} class={pre_class} style={pre_style} onclick = {ctx.link().callback(|_|PagerMsg::Pre)}>{"<"}</a></li>
                {
                    (page.page_start()..= page.page_end()).map(|x|{
                        let onclick = ctx.link().callback(|index:usize|PagerMsg::PageItemClick(index));
                        html!{
                            <PagerItem page_index={x} active={page.is_active(x)} {onclick}/>
                        }
                    }).collect::<Html>()
                }
                // <li><a class="pagination-link is-current" aria-label="Page 46" aria-current="page">{46}</a></li
                <li><a href={format!("javascript:void(0)")} class={next_class} style={next_style} onclick = {ctx.link().callback(|_|PagerMsg::Next)}>{">"}</a></li>
                <li><a href={format!("javascript:void(0)")} class={next_class} style={next_style} onclick = {ctx.link().callback(|_|PagerMsg::Last)}>{">>"}</a></li>
                <li>{"-- total pages:"}<b>{page.page_total()}</b>{" total records: "}<b>{page.total}</b>{" --"}</li>
            </ul>
            </nav>
        }
    }
}
