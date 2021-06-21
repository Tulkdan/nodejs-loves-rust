use neon::prelude::{FunctionContext, Context, JsResult, JsNumber};

pub fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(x + y))
}
