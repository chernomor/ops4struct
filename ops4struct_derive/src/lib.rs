// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://docs.rs/syn/
// https://docs.rs/quote/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Add)]
pub fn add(input: TokenStream) -> TokenStream {
	let (struct_name, fields) = prepare(input);

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
	let (struct_name, fields) = prepare(input);

	let ops = fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! { #ident: self.#ident + other.#ident }
	});

	let tokens = quote! {
		impl AddAssign for #struct_name {
			fn add_assign(&mut self, other: Self) {
				*self = Self {
					#( #ops, )*
				}
			}
		}
	};

	tokens.into()
}

#[proc_macro_derive(Sub)]
pub fn sub(input: TokenStream) -> TokenStream {
	let (struct_name, fields) = prepare(input);

	let ops = fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! { #ident: self.#ident - other.#ident }
	});

	let tokens = quote! {
		impl Sub for #struct_name {
			type Output = Self;
			fn sub(self, other: Self) -> Self {
				Self {
					#( #ops, )*
				}
			}
		}
	};

	tokens.into()
}
#[proc_macro_derive(SubAssign)]
pub fn sub_assign(input: TokenStream) -> TokenStream {
	let (struct_name, fields) = prepare(input);

	let ops = fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! { #ident: self.#ident - other.#ident }
	});

	let tokens = quote! {
		impl SubAssign for #struct_name {
			fn sub_assign(&mut self, other: Self) {
				*self = Self {
					#( #ops, )*
				}
			}
		}
	};

	tokens.into()
}


fn prepare(input: TokenStream)
	-> (proc_macro2::Ident, syn::punctuated::Punctuated<syn::Field, syn::token::Comma>)
{
	let ast: syn::DeriveInput = syn::parse(input).unwrap();
	let struct_name = ast.ident.clone();

	let fields = match ast.data {
		syn::Data::Struct(datastruct) => match datastruct.fields {
			syn::Fields::Named(fields) => {
				fields.named.clone()
			},
			_ => panic!("You can only derive this on structs with named fields!"),
		},
		_ => panic!("You can only derive this on structs with named fields!"),
	};
	(struct_name, fields)
}

