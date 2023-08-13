# Sucrose

A Rust crate for embedding static data from files at build time. 

## Resource

Sucrose works by generating mirrors of existing structures with entirely static fields.

For example, this structure:

```rust
#[derive(sucrose::Resource)]
struct Character {
	name: String
}

Character {
	name: String::from("Hello, world!"),
};
```
...would be translated into this:

```rust
struct Character {
	name: &'static str
}

Character {
	name: "Hello, world!",
};
```

The conversion may look trivial at first, but it allows for a lot of flexibilty in how types are represented.
You can also *nest* resources (types which implement `Resource`),
and create conversions for your own types using the `ToStatic` trait.

## ToStatic

Certain types aren't necessarily resources, but need to be members of a resource regardless.
For example, consider a simple integer type: `i32`.
Sucrose needs to know how to trasform this type from a dynamic to static representation.
In the case of integer, that's simple: both representations are the same.

The `ToStatic` trait can be used to express that a type may be a member of a resource
(in fact, `#derive(Resource` also generates an implementation of `ToStatic`).
An implementation for `i32` may look like this:

```ignore
impl ToStatic for i32 {
	fn static_type() -> TokenStream {
		quote!(i32)
	}
	fn static_value(&self) -> TokenStream {
		quote!(#self)
	}
}
```

This implementation says that the static equivalent of `i32` is, well, `i32`,
and that its value would be written in Rust as a plain integer literal (which is what `quote!(#self)` generates).

Sucrose provides implementations for all integer types,
as well as `String`, `Vec`, `bool`, `Option`, and the `NonZero` integer types.
More implementations may be added as needed

> Note that implementations on types from other libraries can only be written within sucrose or the library owning the type.
> To get around this, you can use "newtype" wrappers to give your code ownership of the type.
> As a more long term solution, [open an issue](https://github.com/eievui5/sucrose/issues) requesting an implementation of the type.
> See [the Rust documentation](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence) for more information.
