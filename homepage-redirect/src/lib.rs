use url::Url;
use worker::*;

#[event(fetch)]
async fn main(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let redirect_to = env
        .var("REDIRECT_TO")
        .map(|var| var.to_string())
        .unwrap_or_else(|_| "https://github.com/utterstep/".to_string());

    Response::redirect(Url::parse(&redirect_to)?)
}
