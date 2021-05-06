//! Traits
use crate::{Config, Round};

/// Callback while submiting a new value
pub trait OnAnswerHandler<T: Config> {
	/// This callback will be called in call `pallet_chainlink_feed::Call::submit`
	fn on_answer(feed: T::FeedId, new_data: Round<T::BlockNumber, T::Value>)
	where
		Self: Sized;
}
