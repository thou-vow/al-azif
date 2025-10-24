use rug::Integer;
use serde::{Deserialize, Serialize};

use crate::IdRegistry;

pub trait RawBehavior: Clone + for<'de> Deserialize<'de> + Send + Serialize + Sync {
	type Rehydrated: RehydratedBehavior<Raw = Self>;

	fn rehydrate(
		self,
		registry: &IdRegistry,
	) -> impl Future<Output = Result<Self::Rehydrated, String>> + Send;
}

pub trait RehydratedBehavior: Clone + Send + Sync {
	type Raw: RawBehavior<Rehydrated = Self>;

	fn raw(self) -> Self::Raw;
}

// Macro to easily define types and implement rehydration for them
#[macro_export]
macro_rules! define_rehydrated {
	($name:ident {
		$($field:ident : $ty:ty),+ $(,)?
	}) => {
		paste::paste! {
			#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
			pub struct [<$name Raw>] {
				$(pub $field: <$ty as $crate::rehydration::RehydratedBehavior>::Raw),+
			}
			impl $crate::rehydration::RawBehavior for [<$name Raw>] {
				type Rehydrated = $name;

				async fn rehydrate(self, registry: &$crate::IdRegistry) -> Result<Self::Rehydrated, String> {
					Ok(Self::Rehydrated {
						$($field: self.$field.rehydrate(registry).await?),+
					})
				}
			}

			#[derive(Clone, Debug)]
			pub struct $name {
				$(pub $field: $ty),+
			}
			impl $crate::rehydration::RehydratedBehavior for $name {
				type Raw = [<$name Raw>];

				fn raw(self) -> Self::Raw {
					Self::Raw {
						$($field: self.$field.raw()),+
					}
				}
			}
		}
	};
}

// These types don't have rehydration
macro_rules! not_rehydrated {
	($($ty:ty),+ $(,)?) => {
		$(
			impl RawBehavior for $ty {
				type Rehydrated = Self;

				async fn rehydrate(self, _registry: &IdRegistry) -> Result<Self::Rehydrated, String> { Ok(self) }
			}
			impl RehydratedBehavior for $ty {
				type Raw = Self;

				fn raw(self) -> Self::Raw { self }
			}
		)+
	};
}
not_rehydrated!(bool, Box<str>, Integer);

// These types are wrappers, implementing rehydration based on their generics
impl<T: RawBehavior> RawBehavior for Option<T> {
	type Rehydrated = Option<T::Rehydrated>;

	async fn rehydrate(self, registry: &IdRegistry) -> Result<Self::Rehydrated, String> {
		if let Some(raw) = self {
			return Ok(Some(raw.rehydrate(registry).await?));
		}
		Ok(None)
	}
}
impl<T: RehydratedBehavior> RehydratedBehavior for Option<T> {
	type Raw = Option<T::Raw>;

	fn raw(self) -> Self::Raw { self.map(RehydratedBehavior::raw) }
}
impl<T: RawBehavior> RawBehavior for Vec<T> {
	type Rehydrated = Vec<T::Rehydrated>;

	async fn rehydrate(self, registry: &IdRegistry) -> Result<Self::Rehydrated, String> {
		let mut resolveds = Vec::new();
		for raw in self {
			resolveds.push(raw.rehydrate(registry).await?)
		}
		Ok(resolveds)
	}
}
impl<T: RehydratedBehavior> RehydratedBehavior for Vec<T> {
	type Raw = Vec<T::Raw>;

	fn raw(self) -> Self::Raw { self.into_iter().map(RehydratedBehavior::raw).collect() }
}
