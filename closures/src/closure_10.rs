use futures::{future::BoxFuture, FutureExt};

async fn process <F> (data: u64, function: F) -> u64
where
    F: FnOnce (u64) -> BoxFuture<'static, u64>,
{
    function (data).await
}

#[tokio::main]
async fn main () {

    let multiply = |x: u64| async move { x * 2 }.boxed();
    let divide = |x: u64| async move { x / 2 }.boxed();

    let result = process(10, multiply).await;
    assert_eq!(20, result);

    let result = process(10, divide).await;
    assert_eq!(5, result);
}