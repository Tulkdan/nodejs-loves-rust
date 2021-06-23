use neon::prelude::*;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

fn compute(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 1..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

pub fn fibonacci_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let cb = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let queue = cx.queue();

    std::thread::spawn(move || {
        let result = compute(n);
        queue.send(move |mut cx| {
            let callback = cb.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![
                cx.null().upcast::<JsValue>(),
                cx.string(result.to_str_radix(10)).upcast()
            ];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });

    Ok(cx.undefined())
}