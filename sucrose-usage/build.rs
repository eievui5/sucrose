use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use sucrose::{convert_dir_as_mod, proc_macro2, quote, Resource, ToStatic, TokenStream};

/// This is a special helper type that you can make to refer to static references by name.
#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ItemReference {
	identifier: String,
}

impl ToStatic for ItemReference {
	fn static_type() -> TokenStream {
		quote!(&'static Item)
	}
	fn static_value(&self) -> TokenStream {
		let data = proc_macro2::Ident::new(&self.identifier, proc_macro2::Span::call_site());
		quote!(&super::items::#data)
	}
}

#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize, Resource)]
struct Npc {
	name: String,
	inventory: Vec<ItemReference>,
}

#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize, Resource)]
struct Item {
	name: String,
	value: u32,
}

fn main() -> anyhow::Result<()> {
	// This is a macro rather than a function because the return type changes
	// depending on which invocation recieves it.
	macro_rules! parse {
		() => {
			// By calling `.unwrap_or(None)`, we ignore parse errors.
			// This is useful for mixing multiple types of files in the same directory.
			// If you would prefer to panic instead, call `.unwrap()` and wrap the result in `Some()`
			|s| toml::from_str(s).unwrap_or(None)
		};
	}

	// Build scripts are supposed to use the `OUT_DIR` environment variables to determine where their
	// resources should be placed.
	let out_dir = PathBuf::from(env::var("OUT_DIR")?);
	let mut o = File::create(out_dir.join("res.rs"))?;

	// If your static types have dependencies, include them here.
	// For example, if you need `NonZero` types:
	// write!(o, "use core::num::*;")?;

	convert_dir_as_mod::<Npc>(&mut o, "res/npcs/", parse!())?;
	convert_dir_as_mod::<Item>(&mut o, "res/items/", parse!())?;

	Ok(())
}
