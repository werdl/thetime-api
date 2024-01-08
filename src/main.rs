use std::net::SocketAddr;
use warp::{Filter, reply::{self, Json}};
use warp::Reply;
use thetime::{Ntp, Time};
use reqwest::Client;

// Function to get the client's IP address
fn get_client_ip(remote_addr: Option<SocketAddr>) -> Option<String> {
    remote_addr.map(|addr| addr.ip().to_string())
}

// Function to get the local timezone offset based on the client's IP
async fn get_local_timezone_offset(client_ip: Option<String>) -> Option<String> {
    println!("Client IP: {:?}", client_ip);
    let api_url = format!("http://worldtimeapi.org/api/ip/{}", client_ip.unwrap_or_default());
    let client = Client::new();

    client.get(&api_url)
        .send()
        .await
        .ok()?
        .json::<serde_json::Value>()
        .await
        .ok()?
        .get("utc_offset")?
        .as_str()
        .map(|tz| tz.to_string())
}

#[tokio::main]
async fn main() {

    let tz = warp::path!("tz" / String).map(|tz: String| reply::json(&Ntp::now().change_tz(tz).pretty()));

    let unix_tz = warp::path!("unix" / String).map(|tz: String| reply::json(&Ntp::now().change_tz(tz).unix()));

    let unix = warp::path!("unix").map(|| reply::json(&Ntp::now().unix()));

    let ip = warp::path!("ip").and(warp::addr::remote()).map(|remote_addr: Option<SocketAddr>| {
        let res = get_client_ip(remote_addr);
        reply::json(&res)
    });


    let cors = warp::cors().allow_any_origin();


    let local = warp::path!("local")
    .and(warp::addr::remote())
    .and_then(move |remote_addr: Option<SocketAddr>| async move {
        let res = get_local_timezone_offset(get_client_ip(remote_addr)).await;
        Ok::<_, warp::Rejection>(res.map_or_else(|| reply::json(&"Error"), |timezone| reply::json(&timezone)))
    })
    .map(|reply: Json| reply.into_response());

warp::serve(tz.or(unix_tz).or(unix).or(local).or(ip).with(cors)).run(([0, 0, 0, 0], 3030)).await;

}
