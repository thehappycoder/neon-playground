#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::prelude::*;

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let array: Handle<JsArray> = cx.argument(0)?;

    let mut result: f64 = 0.0;

    let vec: Vec<Handle<JsValue>> = array.to_vec(&mut cx)?;

    for item in &vec {
        match item.downcast::<JsNumber>().or_throw(&mut cx) {
            Ok(value) => result += value.value(),
            Err(_error) => panic!("number expected"),
        }
    }

    Ok(cx.number(result))
}

fn sum2(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let array: Handle<JsArray> = cx.argument(0)?;

    let mut result: f64 = 0.0;

    for i in 0..array.len() {
        let item = array.get(&mut cx, i)?;

        match item.downcast::<JsNumber>().or_throw(&mut cx) {
            Ok(value) => result += value.value(),
            Err(_error) => panic!("number expected"),
        }
    }

    Ok(cx.number(result))
}

register_module!(mut m, {
    m.export_function("threadCount", thread_count)?;
    m.export_function("sum", sum)?;
    m.export_function("sum2", sum2)?;
    Ok(())
});
