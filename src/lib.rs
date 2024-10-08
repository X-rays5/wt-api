use worker::*;

mod utils;
mod v1;
mod index;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().unwrap().coordinates().unwrap_or_default(),
        req.cf().unwrap().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", index::get_index)
        .on_async("/health", |_, _| async {
            let response = Response::ok("Health check OK");
            response
        })
        .get_async("/v1/vehicles/:country/:category", v1::vehicles::get::country_specific)
        .get_async("/v1/countries", v1::countries::get::countries)
        .get_async("/v1/countries/have/:category", v1::countries::get::have_category)
        .get_async("/v1/categories", v1::categories::get::get_categories)
        .get_async("/v1/categories/has/:category/:country", v1::categories::get::country_has_categories)
        .get_async("/v1/categories/countries", v1::categories::get::which_categories_per_country)

        .get_async("/v1/usercontent", v1::usercontent::categories::router::route_category)
        .run(req, env)
        .await
}
