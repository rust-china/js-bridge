use js_bridge::{args, Context};

fn main() {
    let mut context = Context::default();
    // runtime.run();
    let _res = context
        .eval(
            r#"
            function hello(...values) {
                return "hello " + values.join('');
            }
            "#,
        )
        .unwrap();
    let res = context.call_with_args("hello", args!["world", 2, "!"]).unwrap();
    println!("ret: {:?}", res);
}
