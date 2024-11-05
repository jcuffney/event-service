use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", command),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
