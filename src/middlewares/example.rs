use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    Error,
};

pub async fn example_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) ->  Result<ServiceResponse<impl MessageBody>, Error> {
    println!("Hi from middleware!");
    let res = next.call(req).await?;
    Ok(res)
}