use std::{future::Future, pin::Pin};


async fn process <F> (data: u64, function: F) -> u64
where
    F: FnOnce (u64) -> Pin<Box<dyn Future <Output = u64>>>,
{
    function (data).await
}

#[tokio::main]
async fn main () {

    let multiply = |x: u64| Box::pin (async move { x * 2 }) as Pin<Box<dyn Future <Output = u64>>>;
    let divide = |x: u64| Box::pin (async move { x / 2 }) as Pin<Box<dyn Future <Output = u64>>>;

    let result = process(10, multiply).await;
    assert_eq!(20, result);

    let result = process(10, divide).await;
    assert_eq!(5, result);
}