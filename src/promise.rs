use nodejs_sys::{
    napi_async_work, napi_callback_info, napi_create_async_work, napi_create_error,
    napi_create_function, napi_create_int64, napi_create_promise, napi_create_string_utf8,
    napi_deferred, napi_delete_async_work, napi_env, napi_get_cb_info, napi_get_value_int64,
    napi_queue_async_work, napi_reject_deferred, napi_resolve_deferred, napi_set_named_property,
    napi_status, napi_value,
};
use std::ffi::c_void;
use std::ffi::CString;

// creating struct to sotre a pointer for async_work, deferred and the value
#[derive(Debug, Clone)]
struct Data {
    deferred: napi_deferred,
    work: napi_async_work,
    val: u64,
    result: Option<Result<u64, String>>,
}

pub unsafe extern "C" fn feb(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut buffer: Vec<napi_value> = Vec::with_capacity(1);
    let p = buffer.as_mut_ptr();
    let mut argc = 1 as usize;
    std::mem::forget(buffer);

    // getting arguments from function
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        p,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    let mut start: i64 = 0;

    // converting the napi_value to i64 number
    napi_get_value_int64(env, *p, &mut start);

    // promise which would be returned
    let mut promise: napi_value = std::mem::zeroed();

    // a pointer to promise to resolve is or reject it
    let mut deferred: napi_deferred = std::mem::zeroed();

    // a pointer to our async work name used for debugging
    let mut work_name: napi_value = std::mem::zeroed();

    //pointer to async work
    let mut work: napi_async_work = std::mem::zeroed();

    let async_name = CString::new("async fibonacci").expect("Error creating string");

    // creating a string for name
    napi_create_string_utf8(env, async_name.as_ptr(), 13, &mut work_name);

    // creating a promise
    napi_create_promise(env, &mut deferred, &mut promise);

    let v = Data {
        deferred,
        work,
        val: start as u64,
        result: None,
    };

    // creating a context which can be saved to share state between our functions
    let data = Box::new(v);

    // converting it to raw pointer
    let raw = Box::into_raw(data);

    // creating the work
    napi_create_async_work(
        env,
        std::ptr::null_mut(),
        work_name,
        Some(perform),
        Some(complete),
        std::mem::transmute(raw),
        &mut work,
    );

    // queueing to execute the work
    napi_queue_async_work(env, work);

    // setting pointer to work that can be used later
    (*raw).work = work;

    // returning the promise
    promise
}

// Function that will actually do the work to calculate fibonacci
pub unsafe extern "C" fn perform(_env: napi_env, data: *mut c_void) {
    // getting the shared data and converting in a box
    let mut t: Box<Data> = Box::from_raw(std::mem::transmute(data));
    let mut last = 1;
    let mut second_last = 0;

    for _ in 2..t.val {
        let temp = last;
        last = last + second_last;
        second_last = temp;
    }

    // setting the result on shared context
    t.result = Some(Ok(last));

    // telling the rest to not drop the context data
    Box::into_raw(t);
}

pub unsafe extern "C" fn complete(env: napi_env, _status: napi_status, data: *mut c_void) {
    // getting the shared context
    let mut task: Box<Data> = Box::from_raw(std::mem::transmute(data));

    let v = match task.result {
        Some(d) => match d {
            Ok(result) => result,
            Err(_) => {
                // if there is error, just throw an erro
                // creating error
                let mut js_error: napi_value = std::mem::zeroed();
                napi_create_error(
                    env,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut js_error,
                );

                // rejecting the promise with error
                napi_reject_deferred(env, task.deferred, js_error);

                //deleting the task from the queue
                napi_delete_async_work(env, task.work);
                return;
            }
        },
        None => {
            // if no result is found reject with error
            // creating an error
            let mut js_error: napi_value = std::mem::zeroed();
            napi_create_error(
                env,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut js_error,
            );

            //rejecting promise with error
            napi_reject_deferred(env, task.deferred, js_error);

            // deleting the task from queue
            napi_delete_async_work(env, task.work);

            return;
        }
    };

    // creating the number
    let mut obj: napi_value = std::mem::zeroed();
    napi_create_int64(env, v as i64, &mut obj);

    // resolving the promise with result
    napi_resolve_deferred(env, task.deferred, obj);

    // deleting the work
    napi_delete_async_work(env, task.work);
}

pub unsafe extern "C" fn create_method(env: &napi_env, exports: &napi_value) {
    // creating a function name
    let p = CString::new("myAsyncFunc").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();

    // creating the function
    napi_create_function(
        *env,
        p.as_ptr(),
        5,
        Some(feb),
        std::ptr::null_mut(),
        &mut local,
    );

    // setting function as property
    napi_set_named_property(*env, *exports, p.as_ptr(), local);
}
