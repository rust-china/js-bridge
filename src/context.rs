use deno_core::v8;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

pub struct Context {
    pub js_runtime: JsRuntime,
}

impl Default for Context {
    fn default() -> Self {
        Self::new(RuntimeOptions { extensions: vec![], ..Default::default() })
    }
}

impl Context {
    pub fn new(options: RuntimeOptions) -> Self {
        let js_runtime = JsRuntime::new(options);
        Self { js_runtime }
    }

    pub fn serde_global(&mut self, global: v8::Global<v8::Value>) -> anyhow::Result<serde_json::Value> {
        let scope = &mut self.js_runtime.handle_scope();
        let local = v8::Local::new(scope, global);
        let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local)?;
        Ok(deserialized_value)
    }

    pub fn eval(&mut self, code: &str) -> anyhow::Result<serde_json::Value> {
        let global = self.js_runtime.execute_script("eval", code.to_string())?;
        Ok(self.serde_global(global)?)
    }
    pub fn call_with_args<T: Into<serde_json::Value>>(&mut self, function_name: &str, args: Vec<T>) -> anyhow::Result<serde_json::Value> {
        let context = self.js_runtime.main_context();
        let scope: &mut v8::HandleScope<v8::Context> = &mut self.js_runtime.handle_scope();
        let context_local = v8::Local::new(scope, context);
        let global = context_local.global(scope);

        let function_key = v8::String::new(scope, function_name).unwrap();
        let function_value = global.get(scope, function_key.into()).unwrap();

        let function = v8::Local::<v8::Function>::try_from(function_value).unwrap();
        let args: Vec<serde_json::Value> = args.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        let args = args.iter().map(|x| serde_v8::to_v8(scope, x).unwrap()).collect::<Vec<_>>();

        let ret = function.call(scope, global.into(), &args).unwrap();

        Ok(serde_v8::from_v8::<serde_json::Value>(scope, ret)?)
    }
    pub fn call<T: Into<serde_json::Value>>(&mut self, function_name: &str) -> anyhow::Result<serde_json::Value> {
        self.call_with_args::<T>(function_name, vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_eval() {
        let mut context = Context::default();
        let ret = context
            .eval(
                r#"
                let a = { a: 1, b: 2 };
                a
            "#,
            )
            .unwrap();
        assert_eq!(ret, serde_json::json!({ "a": 1, "b": 2 }));
    }

    #[test]
    fn test_context_call() {
        let mut context = Context::default();
        let _res = context
            .eval(
                r#"
            function hello(...values) {
                return "hello " + values.join('');
            }
            "#,
            )
            .unwrap();
        let ret = context.call_with_args("hello", crate::args!["world", 2, "!"]).unwrap();
        assert_eq!(ret, serde_json::json!("hello world2!"));
    }
}
