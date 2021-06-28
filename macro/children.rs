use quote::quote;

pub fn children(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
	let input: syn::DeriveInput = syn::parse2(input)?;
	let ident = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	match &input.data {
		syn::Data::Struct(_) => {}
		_ => {
			return Err(syn::Error::new_spanned(
				input,
				"this macro can only be used on a struct",
			))
		}
	};
	let child_fn = quote! {
		pub fn child<T>(mut self, child: T) -> Self
		where
			T: Into<Node>
		{
			let child = child.into();
			self.children.push(child);
			self
		}
	};
	let children_fn = quote! {
		pub fn children<T, I>(mut self, children: I) -> Self
		where
			T: Into<Node>, I: IntoIterator<Item = T>
		{
			for child in children {
				let child = child.into();
				self.children.push(child);
			}
			self
		}
	};
	let child_signal_fn = quote! {
		pub fn child_signal<T, S>(mut self, signal: S) -> Self
		where
			T: Into<Node>,
			S: 'static + Unpin + Signal<Item = T>,
		{
			self.children.push(Node::Signal(SignalNode::new(signal)));
			self
		}
	};
	let child_signal_vec_fn = quote! {
		pub fn child_signal_vec<T, S>(mut self, signal_vec: S) -> Self
		where
			T: Into<Node>,
			S: 'static + Unpin + SignalVec<Item = T>,
		{
			self.children.push(Node::SignalVec(SignalVecNode::new(signal_vec)));
			self
		}
	};
	Ok(quote! {
		impl#impl_generics #ident#ty_generics #where_clause {
			#child_fn
			#children_fn
			#child_signal_fn
			#child_signal_vec_fn
		}
	})
}
