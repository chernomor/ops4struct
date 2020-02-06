// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://docs.rs/syn/
// https://docs.rs/quote/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Add)]
pub fn add(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	let struct_name = &ast.ident;

	let fields = match ast.data {
		syn::Data::Struct(datastruct) => match datastruct.fields {
			syn::Fields::Named(fields) => {
				fields.named.clone()
			},
			_ => panic!("You can only derive this on structs with named fields!"),
		},
		_ => panic!("You can only derive this on structs with named fields!"),
	};

	let add_ops = fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! { #ident: self.#ident + other.#ident }
	});

	let tokens = quote! {
		impl Add for #struct_name {
			type Output = Self;
			fn add(self, other: Self) -> Self {
				Self {
					#( #add_ops, )*
				}
			}
		}
	};

	tokens.into()
}
#[proc_macro_derive(AddAssign)]
pub fn add_assign(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	let struct_name = &ast.ident;

	let fields = match ast.data {
		syn::Data::Struct(datastruct) => match datastruct.fields {
			syn::Fields::Named(fields) => {
				fields.named.clone()
			},
			_ => panic!("You can only derive this on structs with named fields!"),
		},
		_ => panic!("You can only derive this on structs with named fields!"),
	};

	let add_ops = fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! { #ident: self.#ident + other.#ident }
	});

	let tokens = quote! {
		impl AddAssign for #struct_name {
			fn add_assign(&mut self, other: Self) {
				*self = Self {
					#( #add_ops, )*
				}
			}
		}
	};

	tokens.into()
}

