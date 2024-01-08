use warp::Filter;
fn main() {
    println!("Hello, world!");
}
#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .map(|| "Hello, world!");

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
