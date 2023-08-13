use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::*;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(resource), supports(struct_named))]
#[allow(dead_code)]
struct ResourceArgs {
	ident: Ident,
	generics: Generics,
	data: ast::Data<(), ResourceFieldArgs>,
}

#[derive(Debug, FromField)]
#[darling(attributes(resource))]
#[allow(dead_code)]
struct ResourceFieldArgs {
	ident: Option<Ident>,
	ty: Type,

	#[darling(default)]
	string: bool,
	#[darling(default)]
	slice: String,
}

struct Interp(Ident);

impl ToTokens for Interp {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Punct::new('#', Spacing::Alone));
		tokens.append(self.0.clone());
	}
}

struct Escape;

impl ToTokens for Escape {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Punct::new('#', Spacing::Alone));
	}
}

fn convert_fields<'a>(fields: &'a [&'a ResourceFieldArgs]) -> (Vec<Ident>, Vec<Interp>) {
	let names: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
	let interp = names.iter().map(|f| Interp(f.clone())).collect();
	(names, interp)
}

/// Generates a `Resource` type by calling `ToStatic` on each of its members
/// and creating a static definition of the struct.
#[proc_macro_derive(Resource)]
pub fn resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	let args = match ResourceArgs::from_derive_input(&input) {
		Ok(v) => v,
		Err(e) => {
			return e.write_errors().into();
		}
	};

	let name = input.ident;

	let fields = args.data.as_ref().take_struct().unwrap().fields;

	let (field_names, field_interpolated_names) = convert_fields(&fields);
	let field_types: Vec<Type> = fields.iter().map(|f| f.ty.clone()).collect();

	quote! {
		impl ::sucrose::ToStatic for #name {
			fn static_type() -> ::sucrose::proc_macro2::TokenStream {
				::sucrose::quote!(#name)
			}

			fn static_value(&self) -> ::sucrose::proc_macro2::TokenStream {
				use ::sucrose::ToStatic;

				#(let #field_names = self.#field_names.static_value();)*

				::sucrose::quote! {
					#name {
						#(#field_names: #field_interpolated_names,)*
					}
				}
			}
		}
		impl ::sucrose::Resource for #name {
			fn static_struct() -> ::sucrose::proc_macro2::TokenStream {
				use ::sucrose::ToStatic;
				#(let #field_names = <#field_types>::static_type();)*

				::sucrose::quote! {
					#[derive(Clone, Debug, Default)]
					pub struct #name {
						#(pub #field_names: #field_interpolated_names,)*
					}
				}
			}
		}
	}
	.into()
}
