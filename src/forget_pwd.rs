use crate::util::common;
use crate::util::error::ErrorKind;
use crate::util::request;
use gloo::timers::callback::Timeout;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const DEFAULT_CODE_BUTTON_TEXT: &str = "Generate Code";
const DEFAULT_CODE_BUTTON_CLASS: &str = "button is-block is-fullwidth is-primary is-medium is-rounded";

#[derive(Serialize)]
pub struct SendEmailCodeReq<'a> {
    email: &'a str,
    from: &'a str,
}

#[derive(Serialize, PartialEq, Clone, Debug, Default)]
pub struct ForgetPwdReq {
    email: String,
    pwd: String,
    code: String,
}

#[derive(Debug)]
pub enum ValidStatus {
    Valid,
    InValid(String),
    None,
}

pub struct ForgetPwd {
    refs: Vec<NodeRef>,
    req: ForgetPwdReq,
    email_valid: ValidStatus,
    pwd_valid: ValidStatus,
    pwd_confirm_valid: ValidStatus,
    code_valid: ValidStatus,
    request_fail_msg: String,
    code_input_disabled: bool,
    code_button_disabled: bool,
    code_button_text: String,
    code_button_class: String,
    password_confirm: String,
    code_fail_msg: String,
}

pub enum ValidateExistEmailOperation {
    Nothing,
    ResetPwd,
    SendEmailCode,
}

pub enum ForgetPwdMsg {
    ValidateExistEmail(ValidateExistEmailOperation),
    ValidateExistEmailSuccess(ValidateExistEmailOperation),
    ValidateExistEmailFail(Box<dyn std::error::Error>),
    EmailChange(web_sys::KeyboardEvent),
    PasswordChange(web_sys::KeyboardEvent),
    PasswordConfirmChange(web_sys::KeyboardEvent),
    CodeChange(web_sys::KeyboardEvent),
    HandleChangePwdSuccess,
    HandleChangePwdError(Box<dyn std::error::Error>),
    KeyDownForgetPwd(web_sys::KeyboardEvent),
    HandleSendEmailCodeSuccess(usize),
    HandleSendEmailCodeError(Box<dyn std::error::Error>),
    HandleSendEmailCodeHint(Box<dyn std::error::Error>),
}

impl Component for ForgetPwd {
    type Message = ForgetPwdMsg;

    type Properties = ();

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let email_input = &self.refs[0];
            email_input
                .cast::<HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            refs: vec![NodeRef::default()],
            req: Default::default(),
            email_valid: ValidStatus::None,
            pwd_valid: ValidStatus::None,
            pwd_confirm_valid: ValidStatus::None,
            code_valid: ValidStatus::None,
            request_fail_msg: Default::default(),
            code_button_text: DEFAULT_CODE_BUTTON_TEXT.to_string(),
            code_input_disabled: true,
            code_button_disabled: true,
            password_confirm: Default::default(),
            code_fail_msg: Default::default(),
            code_button_class: DEFAULT_CODE_BUTTON_CLASS.to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ForgetPwdMsg::ValidateExistEmail(operation) => {
                let email = self.req.email.clone();
                ctx.link().send_future(async move {
                    match common::validate_exist_email(&email).await {
                        Ok(_) => ForgetPwdMsg::ValidateExistEmailSuccess(operation),
                        Err(err) => ForgetPwdMsg::ValidateExistEmailFail(Box::new(err)),
                    }
                });
                false
            }
            ForgetPwdMsg::ValidateExistEmailSuccess(operation) => {
                self.email_valid = ValidStatus::Valid;
                self.code_button_disabled = false;
                match operation {
                    ValidateExistEmailOperation::Nothing => (),
                    ValidateExistEmailOperation::ResetPwd => {
                        if let Err(e) = common::validate_code(&self.req.code) {
                            self.code_valid = ValidStatus::InValid(format!("{}", e));
                            return true;
                        }

                        if let Err(e) = common::validate_pwd(&self.req.pwd) {
                            self.pwd_valid = ValidStatus::InValid(format!("{}", e));
                            return true;
                        }

                        if let Err(e) =
                            common::validate_pwd_confirm(&self.req.pwd, &self.password_confirm)
                        {
                            self.pwd_confirm_valid = ValidStatus::InValid(format!("{}", e));
                            return true;
                        }

                        let req = self.req.clone();
                        ctx.link().send_future(async move {
                            match request::put::<(), _>(request::Host::ApiBase, "/change_pwd", &req)
                                .await
                            {
                                Ok(_) => ForgetPwdMsg::HandleChangePwdSuccess,
                                Err(err) => ForgetPwdMsg::HandleChangePwdError(Box::new(err)),
                            }
                        });
                    }
                    ValidateExistEmailOperation::SendEmailCode => {
                        let email = self.req.email.clone();

                        self.code_button_class = "button is-block is-fullwidth is-primary is-medium is-rounded is-loading".to_string();
                        ctx.link().send_future(async move {
                            match request::post::<usize, _>(
                                request::Host::ApiBase,
                                "/send_email_code",
                                &SendEmailCodeReq {
                                    email: &email,
                                    from: "ChangePwd",
                                },
                            )
                            .await
                            {
                                Ok(res) => {
                                    ForgetPwdMsg::HandleSendEmailCodeSuccess(res.data.unwrap())
                                }
                                Err(err) => match err {
                                    ErrorKind::Hint(_) => {
                                        ForgetPwdMsg::HandleSendEmailCodeHint(Box::new(err))
                                    }
                                    _ => ForgetPwdMsg::HandleSendEmailCodeError(Box::new(err)),
                                },
                            }
                        });
                    }
                }
                true
            }
            ForgetPwdMsg::ValidateExistEmailFail(e) => {
                self.email_valid = ValidStatus::InValid(format!("{}", e));
                self.code_button_disabled = true;
                true
            }
            ForgetPwdMsg::EmailChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                self.req.email = el.value();
                ctx.link().send_message(ForgetPwdMsg::ValidateExistEmail(
                    ValidateExistEmailOperation::Nothing,
                ));
                true
            }
            ForgetPwdMsg::CodeChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                self.req.code = el.value();
                self.code_valid = match common::validate_code(&self.req.code) {
                    Ok(_) => ValidStatus::Valid,
                    Err(e) => ValidStatus::InValid(format!("{}", e)),
                };
                true
            }
            ForgetPwdMsg::PasswordChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                self.req.pwd = el.value();
                self.pwd_valid = match common::validate_pwd(&self.req.pwd) {
                    Ok(_) => ValidStatus::Valid,
                    Err(e) => ValidStatus::InValid(format!("{}", e)),
                };
                true
            }
            ForgetPwdMsg::PasswordConfirmChange(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                self.password_confirm = el.value();
                self.pwd_confirm_valid =
                    match common::validate_pwd_confirm(&self.req.pwd, &self.password_confirm) {
                        Ok(_) => ValidStatus::Valid,
                        Err(e) => ValidStatus::InValid(format!("{}", e)),
                    };
                true
            }

            ForgetPwdMsg::HandleChangePwdSuccess => {
                common::set_local_storage("email", self.req.email.as_str());
                common::set_local_storage("pwd", self.req.pwd.as_str());
                common::redirect("/login");
                false
            }
            ForgetPwdMsg::HandleChangePwdError(e) => {
                self.request_fail_msg = format!("{}", e);
                true
            }
            ForgetPwdMsg::KeyDownForgetPwd(e) => {
                if e.key_code() == 13 {
                    ctx.link().send_message(ForgetPwdMsg::ValidateExistEmail(
                        ValidateExistEmailOperation::ResetPwd,
                    ))
                }
                false
            }
            ForgetPwdMsg::HandleSendEmailCodeSuccess(expired_secs) => {
                self.code_button_class = DEFAULT_CODE_BUTTON_CLASS.to_string();
                if expired_secs > 0 {
                    self.code_button_disabled = true;
                    self.code_input_disabled = false;
                    self.code_button_text = expired_secs.to_string();
                    let link = ctx.link().clone();
                    Timeout::new(1000, move || {
                        link.send_message(ForgetPwdMsg::HandleSendEmailCodeSuccess(expired_secs - 1))
                    })
                    .forget();
                } else {
                    self.code_button_disabled = false;
                    self.code_input_disabled = true;
                    self.code_button_text = DEFAULT_CODE_BUTTON_TEXT.to_string();
                }
                true
            }
            ForgetPwdMsg::HandleSendEmailCodeError(e) => {
                self.code_button_class = DEFAULT_CODE_BUTTON_CLASS.to_string();
                self.code_fail_msg = format!("{}", e);
                true
            }
            ForgetPwdMsg::HandleSendEmailCodeHint(e) => {
                self.code_button_class = DEFAULT_CODE_BUTTON_CLASS.to_string();
                self.code_input_disabled = false;
                self.code_fail_msg = format!("{}", e);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut email_invalid_msg = "";
        let email_input_class = match &self.email_valid {
            ValidStatus::Valid => "input is-success is-medium is-rounded",
            ValidStatus::InValid(e) => {
                email_invalid_msg = e;
                "input is-danger is-medium is-rounded"
            }
            ValidStatus::None => "input is-info is-medium is-rounded",
        };
        let mut pwd_invalid_msg = "";
        let pwd_input_class = match &self.pwd_valid {
            ValidStatus::Valid => "input is-success is-medium is-rounded",
            ValidStatus::InValid(e) => {
                pwd_invalid_msg = e;
                "input is-danger is-medium is-rounded"
            }
            ValidStatus::None => "input is-info is-medium is-rounded",
        };

        let mut pwd_confirm_invalid_msg = "";
        let pwd_confirm_input_class = match &self.pwd_confirm_valid {
            ValidStatus::Valid => "input is-success is-medium is-rounded",
            ValidStatus::InValid(e) => {
                pwd_confirm_invalid_msg = e;
                "input is-danger is-medium is-rounded"
            }
            ValidStatus::None => "input is-info is-medium is-rounded",
        };

        let mut code_fail_msg = &self.code_fail_msg;
        let code_input_class = match &self.code_valid {
            ValidStatus::Valid => "input is-success is-medium is-rounded",
            ValidStatus::InValid(e) => {
                code_fail_msg = e;
                "input is-danger is-medium is-rounded"
            }
            ValidStatus::None => "input is-info is-medium is-rounded",
        };
        html! {
            <>
            <header>
                <link rel="stylesheet" type="text/css" href="/forget_pwd.css"/>
            </header>
            <section class="hero is-fullheight">
                <div class="hero-body">
                    <div class="forget_pwd">
                        <div class="field has-text-centered">
                            <img alt="fuck you" src="/static/img/logo.png" style="height: 100px"/>
                        </div>
                        <div class="field">
                            <label class="label">{"Email:"}</label>
                        </div>
                        <div class="field is-horizontal">
                            <div class="field-body">
                                <div class="field is-expanded">
                                    <div class="control has-icons-left">
                                    <input ref={&self.refs[0]} class={email_input_class} type="email" onkeyup={ctx.link().callback(|e:web_sys::KeyboardEvent|ForgetPwdMsg::EmailChange(e))}   placeholder="hello@example.com"/>
                                        <span class="icon is-small is-left">
                                        <i class="fa-solid fa-envelope"></i>
                                        </span>
                                    </div>
                                </div>
                                <div class="control">
                                    <button disabled = {self.code_button_disabled} class = {&self.code_button_class} onclick={ctx.link().callback(|_|ForgetPwdMsg::ValidateExistEmail(ValidateExistEmailOperation::SendEmailCode))}>
                                            { &self.code_button_text }
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="field">
                            <p class="help is-danger">
                                {email_invalid_msg}
                            </p>
                        </div>
                        <div class="field">
                            <label class="label">{"Code:"}</label>
                            <p class="control has-icons-left">
                                <input class={code_input_class} disabled={self.code_input_disabled} type="text" onkeyup={ctx.link().callback(|e:web_sys::KeyboardEvent|ForgetPwdMsg::CodeChange(e))} placeholder="123456"/>
                                <span class="icon is-small is-left">
                                <i class="fa-solid fa-barcode"></i>
                                </span>
                            </p>

                            <p class="help is-danger">
                                {code_fail_msg}
                            </p>
                        </div>
                        <div class="field">
                            <label class="label">{"New Password:"}</label>
                            <p class="control has-icons-left">
                                <input class={pwd_input_class} type="password" onkeyup={ctx.link().callback(|e:web_sys::KeyboardEvent|ForgetPwdMsg::PasswordChange(e))} placeholder="**********"/>
                                <span class="icon is-small is-left">
                                <i class="fas fa-lock"></i>
                                </span>
                            </p>
                            <p class="help is-danger">
                                {pwd_invalid_msg}
                            </p>
                        </div>

                        <div class="field">
                            <label class="label">{"Re-enter New Password:"}</label>
                            <p class="control has-icons-left">
                                <input class={pwd_confirm_input_class} type="password" onkeyup={ctx.link().callback(|e:web_sys::KeyboardEvent|ForgetPwdMsg::PasswordConfirmChange(e))} onkeydown={ctx.link().callback(|e:web_sys::KeyboardEvent|ForgetPwdMsg::KeyDownForgetPwd(e))} placeholder="**********"/>
                                <span class="icon is-small is-left">
                                <i class="fas fa-lock"></i>
                                </span>
                            </p>
                            <p class="help is-danger">
                                {pwd_confirm_invalid_msg}
                            </p>
                        </div>
                        <br/>
                        <div class="field">
                            <p class="control">
                                <button class="button is-block is-fullwidth is-primary is-medium is-rounded" onclick={ctx.link().callback(|_|ForgetPwdMsg::ValidateExistEmail(ValidateExistEmailOperation::ResetPwd))}>
                                {"Reset Password"}
                                </button>
                            </p>
                            <p class="help is-danger">
                                {self.request_fail_msg.clone()}
                            </p>
                        </div>
                        <br/>
                        <nav class="level">
                        <div class="level-item has-text-centered">
                            <div>
                            <a href="/login">{"Return to login"}</a>
                            </div>
                        </div>
                        </nav>
                    </div>
                </div>
            </section>
            </>
        }
    }
}
