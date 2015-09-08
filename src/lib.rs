extern crate eventual;

pub use eventual::*;

#[macro_export]
macro_rules! async {
    ($e: expr) => {
        Future::spawn(move || { $e })
    };
    ($block:block) => {
        Future::spawn(move || { $block })
    }
}

#[macro_export]
macro_rules! await {
    ($f: expr) => {
        $f.await().unwrap()
    };
    ($f: expr, $d: expr) => {
        match $f.await() {
            Ok(e) => e,
            Err(_) => $d
        }
    }
}

#[test]
fn test_simple_async() {
    let a = async!{42};
    assert_eq!(a.await().unwrap(), 42);
}

#[test]
fn test_complex_async() {
    let f1 = async!{42};
    let f2 = async!{18};
    let transformation = join((f1.map(|v| v * 2), f2.map(|v| v + 5)))
        .and_then(|(v1, v2)| Ok(v1 - v2));
    assert_eq!(61, await!{transformation});
}

#[test]
fn test_block() {
    let f1 = async!{{
        let f1 = async!{42};
        await!{f1.map(|v| v * 2)}
    }};
    assert_eq!(84, await!{f1})
}

#[test]
fn test_await() {
    let a = async!{42};
    assert_eq!(await!(a), 42);
}

#[test]
fn test_default() {
    let a = async!{panic!("i")};
    let res = await!(a, 9711);
    assert_eq!(res, 9711);
}
