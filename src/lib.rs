mod context;
pub use context::Context;

#[macro_export]
macro_rules! args {
	() => {
			Vec::<serde_json::Value>::new()
	};
	($($val:expr),*) => {{
			vec![
					$(
							serde_json::Value::from($val),
					)*
			]
	}};
}
