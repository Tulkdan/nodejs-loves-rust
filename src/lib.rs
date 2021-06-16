use nodejs_sys::{napi_env, napi_value};

pub mod hello_world;
pub mod function;
pub mod promise;

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {

    hello_world::create_method(&env, &exports);

    function::create_method(&env, &exports);

    promise::create_method(&env, &exports);

    // returning the object
    exports
}
