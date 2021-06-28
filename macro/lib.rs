mod builder;
mod children;
mod new;

#[proc_macro_derive(builder, attributes(builder))]
pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	builder::builder(input.into())
		.unwrap_or_else(|e| e.to_compile_error())
		.into()
}

#[proc_macro_derive(children)]
pub fn children(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	children::children(input.into())
		.unwrap_or_else(|e| e.to_compile_error())
		.into()
}

#[proc_macro_derive(new, attributes(new))]
pub fn new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	new::new(input.into())
		.unwrap_or_else(|e| e.to_compile_error())
		.into()
}
