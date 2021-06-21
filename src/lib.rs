use neon::prelude::*;

pub mod hello_world;
pub mod function;
pub mod promise;

register_module!(mut m, {
    m.export_function("hello", hello_world::hello)?;
    m.export_function("add", function::add)?;
    m.export_function("myAsyncFunc", promise::fibonacci_async)?;

    Ok(())
});
