use tide::Request;
use types::AppState;

mod routes;
mod types;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::with_state(AppState::new()?);
    app.at("/").get(routes::get_index);
    app.at("/file/:file").get(routes::get_file);
    app.at("/file/:file")
        .put(move |req: Request<AppState>| routes::put_file(req));

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
