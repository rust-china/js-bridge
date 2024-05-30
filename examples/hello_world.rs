use js_bridge::Context;

fn main() {
    let mut context = Context::default();
    // runtime.run();
    let res = context
        .eval(
            r#"
            function print(value) {
                Deno.core.print(value.toString()+"\n");
            }
            print("is");
            console.log(1234);
            console.log(1+1);
            let a = { a: 1, b: 2 };
            "#,
        )
        .unwrap();
    println!("ret: {:?}", res);
}
