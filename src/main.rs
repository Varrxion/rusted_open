use api::api_controller::ApiEntryPoint;

mod engine;
mod api;

fn main() {
    let mut api = ApiEntryPoint::new();
    api.init();
}
