//! Types
use codec::{Decode, Encode};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::TypeId;

/// Convert a number into an account
#[derive(Encode, Decode, Default)]
pub struct AccountIdConverter<T: BaseArithmetic>(T);

impl<T: BaseArithmetic> From<T> for AccountIdConverter<T> {
	fn from(n: T) -> AccountIdConverter<T> {
		AccountIdConverter(n)
	}
}

impl<T: BaseArithmetic> TypeId for AccountIdConverter<T> {
	const TYPE_ID: [u8; 4] = *b"feed";
}
