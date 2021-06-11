# Derive ID

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/derive-id.svg)](https://crates.io/crates/derive-id)
[![Documentation](https://docs.rs/derive-id/badge.svg)](https://docs.rs/derive-id)

A one size fits some NewType ID derive specifically tailored to integrate with Async GraphQL and Diesel. In particular, this allows for ID types that can be used in conjunction with [`diesel::associations::BelongsTo`](http://docs.diesel.rs/diesel/associations/trait.BelongsTo.html).

```rust
	derive_id! {
		#[derive(Identifiable)]
		#[table_name = "content"]
		#[graphql(name = "ContentID")]
		pub struct ContentId(#[column_name = "id"] i32);
	}
```
