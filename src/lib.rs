#[allow(unused_imports)]
#[allow(unused_macros)]
#[cfg(test)]
#[macro_use]
extern crate diesel;

pub extern crate async_graphql;
pub extern crate derive_more;
pub extern crate diesel_derive_newtype;
pub extern crate serde;

#[macro_export]
macro_rules! derive_id {
	($id_newtype:item) => {
		#[derive(
			$crate::async_graphql::NewType,
			$crate::derive_more::Display,
			$crate::diesel_derive_newtype::DieselNewType,
			$crate::serde::Deserialize,
			$crate::serde::Serialize,
			Clone,
			Copy,
			Debug,
			Eq,
			Hash,
			Ord,
			PartialEq,
			PartialOrd,
		)]
		$id_newtype
	};
}

#[cfg(test)]
mod tests {
	table! {

			content (id) {
					id -> Int4,
					title -> Varchar,
			}
	}

	table! {

			users (id) {
					id -> Int4,
					name -> Varchar,
			}
	}

	table! {

			users_content (user_id, content_id) {
					user_id -> Int4,
					content_id -> Int4,
			}
	}

	derive_id! {
		#[derive(Identifiable)]
		#[table_name = "users"]
		#[graphql(name = "UserID")]
		pub struct UserId(#[column_name = "id"] i32);
	}

	derive_id! {
		#[derive(Identifiable)]
		#[table_name = "content"]
		#[graphql(name = "ContentID")]
		pub struct ContentId(#[column_name = "id"] i32);
	}
}
