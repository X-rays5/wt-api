use worker::{Request, Response, RouteContext};

const RESPONSE: &str = "<h1>All available routes</h1>\n
<a href=\"https://github.com/X-rays5/wt-api\">Gtihub</a>

<h3>INDEX</h3>
<uL>
    <li>GET /               this page</li>
</ul>

<h3>VEHICLE PAGES</h3>
<ul>
    <li>GET /v1/vehicles/:country/:category,:category           Get specific categories from country</li>
    <li>GET /v1/vehicles/:country/all                           Get all categories of vehicles from country</li>
    <li>GET /v1/vehicles/all/:category,:category                Get categories of vehicle from all countries</li>
    <li>GET /v1/vehicles/all/all                                Get all categories from all countries</li>
    <li>GET /v1/categories                                      Get all existing categories</li>
    <li>GET /v1/categories/has/:category,:category/:country     Check if country has categories</li>
    <li>GET /v1/categories/countries                            Get all categories a country has</li>
    <li>GET /v1/countries                                       Get all existing countries</li>
    <li>GET /v1/countries/have/:category                        Get all countries with certain category</li>
</ul>

<h3>USER CONTENT PAGES</h3>
<ul>
    <li>GET /v1/usercontent?user=&period=&searchString= Get user content</li>
</ul>
";

pub fn get_index(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_html(RESPONSE)
}