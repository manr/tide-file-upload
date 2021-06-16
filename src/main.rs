use tide::Request;
use types::AppState;

mod endpoints;
mod types;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::with_state(AppState::new()?);
    app.at("/").get(endpoints::get_index);
    app.at("/file/:file").get(endpoints::get_file);
    app.at("/file/:file")
        .put(move |req: Request<AppState>| endpoints::put_file(req));

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
