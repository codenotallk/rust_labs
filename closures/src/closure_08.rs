
fn process <F> (data: u64, function: F) -> u64
where
    F: FnOnce (u64) -> u64,
{
    function (data)
}

fn main () {

    let multiply = move |x: u64| x * 2;
    let divide = move |x: u64| x / 2;

    let result = process(10, multiply);
    assert_eq!(20, result);

    let result = process(10, divide);
    assert_eq!(5, result);
}