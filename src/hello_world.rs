use neon::prelude::{FunctionContext, Context, JsResult, JsString};

pub fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("world!"))
}

