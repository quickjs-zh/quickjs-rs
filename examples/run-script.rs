use std::convert::TryInto;

// examples/run-script.rs
extern crate quickjs_rs;

use quickjs_rs::*;

fn main() {
	unsafe {
		let rt = JS_NewRuntime();
		let ctx = JS_NewContext(rt);
		let jsmodnfunc:JSModuleNormalizeFunc = None;
		const NULL:* mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;

		// 加载ES6模块
		JS_SetModuleLoaderFunc(rt, jsmodnfunc, Some(js_module_loader), NULL);

		// 加载标准模块
		js_init_module_std(ctx, "std".as_ptr() as *const i8);
    	js_init_module_os(ctx, "os".as_ptr() as *const i8);

    	// 这里尝试执行一行简单的数字相加的代码
    	let code = "8+7;";

    	// 获取到JSValue结构
    	let val = JS_Eval(ctx, code.as_ptr() as *const i8, code.len(), "<cmdline>".as_ptr() as *const i8, JS_EVAL_FLAG_SHEBANG.try_into().unwrap());
    	let i = JS_IsNumber(val);
    	if i > 0 {
    		println!("is number:{}", i);
    		println!("number is :{}", val.u.int32);
    	}
    	js_std_loop(ctx);

    	// 释放相关资源
    	js_std_free_handlers(rt);
    	JS_FreeContext(ctx);
    	JS_FreeRuntime(rt);
	}
	
    println!("Hello from an quickjs_rs example!");
}