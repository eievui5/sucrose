use proc_macro2::TokenStream;
use quote::quote;
use std::num::*;

/// A type that can be converted to a static type.
///
/// This is distinct from `Resource` in that the static type *must already exist*,
/// whereas `Resource` types will generate a definition for themselves.
pub trait ToStatic {
	/// The name of this type's static counterpart.
	/// For `Resource` types, this is the name of the resource.
	///
	/// For example, the `String` type's static type would be `&'static str`
	fn static_type() -> TokenStream;
	/// This type's value converted to a static literal.
	///
	/// For example, the `String` type's static value would be a string literal,
	/// which has a type of `&'static str`
	///
	/// `String::from("Hello, world!")` becomes `"Hello, world!"`.
	fn static_value(&self) -> TokenStream;
}

macro_rules! plain_static {
    ($type:ident, $($remaining:ident),+ $(,)?) => {
        plain_static!($type);
        plain_static!($($remaining),+);
    };
    ($type:ident $(,)?) => {
        impl ToStatic for $type {
            fn static_type() -> TokenStream {
                quote!($type)
            }
            fn static_value(&self) -> TokenStream {
                quote!(#self)
            }
        }
    };
}

macro_rules! nonzero_static {
    ($type:ident, $($remaining:ident),+ $(,)?) => {
        nonzero_static!($type);
        nonzero_static!($($remaining),+);
    };
    ($type:ident $(,)?) => {
        impl ToStatic for $type {
            fn static_type() -> TokenStream {
                quote!(::core::num::$type)
            }
            fn static_value(&self) -> TokenStream {
                let value = self.get();
                // This value is generated using a NonZero type.
                // Under no circumstance will it be 0.
                // using the unsafe method is necessary because Option::unwrap is not const
                quote!(unsafe { ::core::num::$type::new_unchecked(#value) })
            }
        }
    };
}

plain_static!(u8, u16, u32, u64);
plain_static!(i8, i16, i32, i64);
plain_static!(bool);

nonzero_static!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64);
nonzero_static!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64);

impl ToStatic for String {
	fn static_type() -> TokenStream {
		quote!(&'static str)
	}
	fn static_value(&self) -> TokenStream {
		quote!(#self)
	}
}

impl<T: ToStatic> ToStatic for Vec<T> {
	fn static_type() -> TokenStream {
		let ty = T::static_type();
		quote!(&'static [#ty])
	}
	fn static_value(&self) -> TokenStream {
		let iter = self.iter().map(ToStatic::static_value);
		quote! {
			&[#(#iter),*]
		}
	}
}

impl<T: ToStatic> ToStatic for Option<T> {
	fn static_type() -> TokenStream {
		let ty = T::static_type();
		quote!(Option<#ty>)
	}
	fn static_value(&self) -> TokenStream {
		match self {
			Some(thing) => {
				let thing = thing.static_value();
				quote!(Some(#thing))
			}
			None => quote!(None),
		}
	}
}
