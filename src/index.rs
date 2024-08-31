use worker::{Request, Response, RouteContext};

const RESPONSE: &str = "All available routes\n
GET /               this page
";

pub fn get_index(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::ok(RESPONSE)
}