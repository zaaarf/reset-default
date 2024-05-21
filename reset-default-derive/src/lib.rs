extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ResetDefault)]
pub fn reset_default_fn(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as syn::DeriveInput);
	let input_type = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	if let syn::Data::Struct(ref struct_data) = input.data {
		let field_names = struct_data.fields
			.iter()
			.filter_map(|f| f.ident.clone()) // TODO: avoid cloning
			.collect::<Vec<syn::Ident>>();
		quote! {
			#[allow(missing_docs)]
			#[automatically_derived]
			impl #impl_generics reset_default::ResetDefault for #input_type #ty_generics #where_clause {
				#[inline]
				fn reset(&mut self) {
					#(self.#field_names = Default::default();)*
				}
			}
		}.into()
	} else {
		panic!("Only structs can reset!r")
	}
}
