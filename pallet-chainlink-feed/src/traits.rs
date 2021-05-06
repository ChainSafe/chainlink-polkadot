//! Traits
use crate::{Config, Round};

pub trait OnAnswerHandler<T: Config> {
	fn on_answer(feed: T::FeedId, new_data: Round<T::BlockNumber, T::Value>);
}

impl<T: Config> OnAnswerHandler<T> for () {
	fn on_answer(_feed: T::FeedId, _new_data: Round<T::BlockNumber, T::Value>) {
		// do_nothing
	}
}
