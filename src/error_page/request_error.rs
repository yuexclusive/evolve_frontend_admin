use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RequestProps {
    pub status: u16,
}

#[function_component(RequestError)]
pub fn request_error(props: &RequestProps) -> Html {
    match props.status {
        401 => html! {
        <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <title>{"401 Unauthorized"}</title>
        </head>
        <body align="center">
            <div role="main" align="center">
                <h1>{"401: Unauthorized"}</h1>
                <p>{"The request requires user authentication."}</p>
                // <p>{"uri: "}<b></b></p>
                <hr/>
            </div>
            <div role="contentinfo" align="center">
                <span style="margin-top:15px;"><a href="/login">{"Go to login"}</a></span>
            </div>
        </body>
        </html>
            },
        404 => html! {
        <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <title>{"401 Unauthorized"}</title>
        </head>
        <body align="center">
            <div role="main" align="center">
                <h1>{"404: Not Found"}</h1>
                <p>{"The requested resource could not be found."}</p>
                // <p>{"uri: "}<b></b></p>
                <hr/>
            </div>
            <div role="contentinfo" align="center">
                <span style="margin-top:15px;"><a href="/login">{"Go to login"}</a></span>
            </div>
        </body>
        </html>
            },
        _ => html! {
        <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <title>{"request api error"}</title>
        </head>
        <body align="center">
            <div role="main" align="center">
                <h1>{"request api error"}</h1>
                <hr/>
            </div>
            <div role="contentinfo" align="center">
                <span style="margin-top:15px;"><a href="/login">{"Go to login"}</a></span>
            </div>
        </body>
        </html>
            },
    }
}
