use crate::util::common;
use crate::util::error::ToError;
use common::BasicResult;
use gloo_net::http::{Method, Request};
use gloo_net::websocket::futures::WebSocket;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize)]
pub struct ResultData<T> {
    pub data: Option<T>,
    pub msg: Option<String>,
    pub total: Option<usize>,
}

#[allow(unused)]
pub enum Host {
    ApiBase,
    Base,
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::ApiBase => f.write_str("http://localhost:8881/api"),
            Host::Base => f.write_str("http://localhost:8881"),
        }
    }
}

fn build_header(req: Request) -> Request {
    let req = req.header("Content-type", "application/json");
    match common::get_token() {
        Ok(v) => req.header("token", format!("Bearer {}", v).as_str()),
        Err(_) => req,
    }
}

fn build_request(method: Method, host: Host, path: &str) -> Request {
    let url = &format!("{}{}", host, path);
    let req = Request::new(url).method(method);
    build_header(req)
}

async fn send<Res>(req: gloo_net::http::Request) -> BasicResult<ResultData<Res>>
where
    Res: DeserializeOwned,
{
    let response = req.send().await?;
    let status = response.status();
    // let url = response.url();
    if status == 401 || status == 404 {
        common::redirect(&format!("/{status}"));

        return Ok(ResultData {
            data: None,
            msg: None,
            total: None,
        });
    }
    let status_first = status / 100;
    let result: ResultData<Res> = response.json().await.map_err(|e| {
        log::error!("json umarshal error: {}", e);
        e
    })?;
    match status_first {
        4 | 5 => match status {
            452 => Err(result.msg.unwrap().to_hint()),
            _ => Err(result.msg.unwrap().to_server_error()),
        },
        _ => Ok(result),
    }
}

pub fn open_ws() -> BasicResult<WebSocket> {
    let token = common::get_token().unwrap();
    spawn_local(async move {
        get::<common::CurrentUser, Vec<(&str, &str)>, _>(
            Host::ApiBase,
            "/user/get_current_user",
            None,
        )
        .await
        .unwrap();
    });

    let url = format!("ws://localhost:8881/ws/ws/{}", token);
    let ws = WebSocket::open(&url)
        .map_err(|err| log::info!("open ws error: {:#?}", err))
        .unwrap();
    Ok(ws)
}

pub async fn get<'a, Res, Param, V>(
    host: Host,
    path: &str,
    params: Option<Param>,
) -> BasicResult<ResultData<Res>>
where
    Param: IntoIterator<Item = (&'a str, V)>,
    Res: DeserializeOwned,
    V: AsRef<str>,
{
    let req = match params {
        Some(p) => build_request(Method::GET, host, path).query(p),
        None => build_request(Method::GET, host, path),
    };

    send(req).await
}

#[allow(unused)]
pub async fn put<'a, Res, Body>(host: Host, path: &str, body: &Body) -> BasicResult<ResultData<Res>>
where
    Body: Serialize,
    Res: DeserializeOwned,
{
    let body = serde_json::to_string(body)?;
    let req = build_request(Method::PUT, host, path).body(body);
    send(req).await
}

#[allow(unused)]
pub async fn post<'a, Res, Body>(
    host: Host,
    path: &str,
    body: &Body,
) -> BasicResult<ResultData<Res>>
where
    Body: Serialize,
    Res: DeserializeOwned,
{
    let body = serde_json::to_string(body)?;
    let req = build_request(Method::POST, host, path).body(body);
    send(req).await
}

#[allow(unused)]
pub async fn delete<'a, Res, Body>(
    host: Host,
    path: &str,
    body: &Body,
) -> BasicResult<ResultData<Res>>
where
    Body: Serialize,
    Res: DeserializeOwned,
{
    let body = serde_json::to_string(body)?;
    let req = build_request(Method::DELETE, host, path).body(body);
    send(req).await
}
