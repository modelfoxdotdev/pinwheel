use quote::quote;

pub fn builder(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
	let input: syn::DeriveInput = syn::parse2(input)?;
	let ident = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	let data = match &input.data {
		syn::Data::Struct(data) => data,
		_ => {
			return Err(syn::Error::new_spanned(
				input,
				"this macro can only be used on a struct",
			))
		}
	};
	let fns = data
		.fields
		.iter()
		.filter_map(|field| {
			let field_ident = &field.ident;
			let field_ty = &field.ty;
			if field.attrs.iter().any(|attr| attr.path.is_ident("builder")) {
				Some((field_ident, field_ty))
			} else {
				None
			}
		})
		.map(|(field_ident, field_ty)| {
			quote! {
				pub fn #field_ident(mut self, #field_ident: impl Into<#field_ty>) -> #ident#ty_generics {
					self.#field_ident = #field_ident.into();
					self
				}
			}
		});
	Ok(quote! {
		impl#impl_generics #ident#ty_generics #where_clause {
			#(#fns)*
		}
	})
}
