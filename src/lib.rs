use worker::{
    wasm_bindgen::{prelude::wasm_bindgen, JsValue},
    *,
};

#[wasm_bindgen(module = "/js/d1.js")]
extern "C" {
    async fn fetch_customers(env: &Env, query: &str, customer_id: &str) -> JsValue;
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get_async("/customers/:customer_id", |_, ctx| async move {
            if let Some(customer_id) = ctx.param("customer_id") {
                return match fetch_customers(
                    &ctx.env,
                    "SELECT * FROM Customers WHERE CustomerID = ?",
                    customer_id,
                )
                .await
                .as_string()
                {
                    Some(result) => Response::ok(result),
                    None => Response::error("Internal Server Error", 500),
                };
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
