use quote::quote;
use syn::spanned::Spanned;

pub fn new(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
	let input: syn::DeriveInput = syn::parse2(input)?;
	let ident = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	let data_struct = match &input.data {
		syn::Data::Struct(data) => data,
		_ => {
			return Err(syn::Error::new(
				input.span(),
				"this macro can only be used on a struct",
			))
		}
	};
	let struct_attrs = attrs(&input)?;
	if struct_attrs.default {
		return Ok(quote! {
			impl #ident {
				pub fn new() -> #ident {
					#ident::default()
				}
			}
		});
	}
	let mut args = Vec::new();
	let mut arg_fields = Vec::new();
	let mut default_fields = Vec::new();
	for field in data_struct.fields.iter() {
		let field_ident = &field.ident;
		let field_ty = &field.ty;
		let field_attrs = field_attrs(field)?;
		match field_attrs.default {
			None => {
				args.push(quote! {
					#field_ident: #field_ty
				});
				arg_fields.push(quote! {
					#field_ident: #field_ident.into()
				});
			}
			Some(FieldDefault::Default) => {
				default_fields.push(quote! {
					#field_ident: Default::default()
				});
			}
			Some(FieldDefault::DefaultLit(lit)) => {
				default_fields.push(quote! {
					#field_ident: #lit
				});
			}
		};
	}
	Ok(quote! {
		impl#impl_generics #ident#ty_generics #where_clause {
			pub fn new(#(#args),*) -> #ident#ty_generics {
				#ident {
					#(#arg_fields,)*
					#(#default_fields,)*
				}
			}
		}
	})
}

struct StructAttrs {
	default: bool,
}

fn attrs(input: &syn::DeriveInput) -> syn::Result<StructAttrs> {
	let attr = input.attrs.iter().find(|attr| attr.path.is_ident("new"));
	let attr = if let Some(attr) = attr {
		Some(attr.parse_meta()?)
	} else {
		None
	};
	let mut default = None;
	if let Some(attr) = attr {
		let list = match attr {
			syn::Meta::List(list) => list,
			_ => {
				return Err(syn::Error::new(
					input.span(),
					"expected attribute to be a list",
				))
			}
		};
		for item in list.nested.iter() {
			match item {
				syn::NestedMeta::Meta(syn::Meta::Path(path)) if path.is_ident("default") => {
					default = Some(true)
				}
				_ => {}
			};
		}
	}
	let default = default.unwrap_or(false);
	Ok(StructAttrs { default })
}

struct FieldAttrs {
	default: Option<FieldDefault>,
}

enum FieldDefault {
	Default,
	DefaultLit(syn::Lit),
}

fn field_attrs(input: &syn::Field) -> syn::Result<FieldAttrs> {
	let attr = input.attrs.iter().find(|attr| attr.path.is_ident("new"));
	let attr = if let Some(attr) = attr {
		Some(attr.parse_meta()?)
	} else {
		None
	};
	let mut default = None;
	if let Some(attr) = attr {
		let list = match attr {
			syn::Meta::List(list) => list,
			_ => {
				return Err(syn::Error::new(
					input.span(),
					"expected attribute to be a list",
				))
			}
		};
		for item in list.nested.iter() {
			match item {
				syn::NestedMeta::Meta(syn::Meta::Path(path)) if path.is_ident("default") => {
					default = Some(FieldDefault::Default);
				}
				syn::NestedMeta::Meta(syn::Meta::NameValue(name_value))
					if name_value.path.is_ident("default") =>
				{
					default = Some(FieldDefault::DefaultLit(name_value.lit.clone()));
				}
				_ => {}
			};
		}
	}
	Ok(FieldAttrs { default })
}
