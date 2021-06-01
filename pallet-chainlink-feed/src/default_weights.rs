//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use crate::{Config, Feed, RoundId, SubmitterPaysFee};
use codec::Encode;
use frame_support::{
	traits::Get,
	weights::{
		constants::RocksDbWeight as DbWeight, ClassifyDispatch, DispatchClass, Pays, PaysFee,
		WeighData, Weight,
	},
};
use frame_system::{ensure_signed, pallet_prelude::OriginFor};

/// Weight of `submit`
pub struct SubmitWeight<T: Config> {
	/// Custom weight
	pub weight: Weight,
	/// Oracle account
	pub oracle: T::AccountId,
	/// Feed ID
	pub feed_id: T::FeedId,
	/// Round ID
	pub round_id: RoundId,
}

impl<T: Config> SubmitWeight<T> {
	pub fn new(
		weight: Weight,
		oracle: T::AccountId,
		feed_id: T::FeedId,
		round_id: RoundId,
	) -> Self {
		Self {
			weight,
			oracle,
			feed_id,
			round_id,
		}
	}
}

impl<T: Encode, C: Config> WeighData<T> for SubmitWeight<C> {
	fn weigh_data(&self, _target: T) -> Weight {
		self.weight
	}
}

impl<T: Encode, C: Config> ClassifyDispatch<T> for SubmitWeight<C> {
	fn classify_dispatch(&self, _target: T) -> DispatchClass {
		DispatchClass::Operational
	}
}

impl<T: Encode, C: Config> PaysFee<T> for SubmitWeight<C> {
	fn pays_fee(&self, _target: T) -> Pays {
		match C::SubmitterPaysFee::get() {
			SubmitterPaysFee::Always => Pays::Yes,
			SubmitterPaysFee::FreeForValidRound => <Feed<C>>::load_from(self.feed_id)
				.map(|feed| {
					if feed.ensure_valid_round(&self.oracle, self.round_id).is_ok() {
						Pays::Yes
					} else {
						Pays::No
					}
				})
				.unwrap_or(Pays::No),
		}
	}
}

impl crate::WeightInfo for () {
	fn create_feed(o: u32) -> Weight {
		(305_438_000 as Weight)
			.saturating_add((62_577_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(o as Weight)))
			.saturating_add(DbWeight::get().writes(3 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(o as Weight)))
	}
	fn transfer_ownership() -> Weight {
		(79_760_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_ownership() -> Weight {
		(77_660_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn submit_opening_round_answers() -> Weight {
		(442_347_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	fn submit_closing_answer(o: u32) -> Weight {
		(362_642_000 as Weight)
			.saturating_add((1_219_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	fn change_oracles(d: u32, n: u32) -> Weight {
		(0 as Weight)
			.saturating_add((74_485_000 as Weight).saturating_mul(d as Weight))
			.saturating_add((85_010_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
	}
	fn update_future_rounds() -> Weight {
		(87_446_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn set_requester() -> Weight {
		(99_375_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_requester() -> Weight {
		(90_710_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn request_new_round() -> Weight {
		(245_256_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn withdraw_payment() -> Weight {
		(210_562_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn transfer_admin() -> Weight {
		(78_330_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_admin() -> Weight {
		(77_813_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn withdraw_funds() -> Weight {
		(183_123_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn reduce_debt() -> Weight {
		(118_115_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn transfer_pallet_admin() -> Weight {
		(64_232_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_pallet_admin() -> Weight {
		(70_817_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn set_feed_creator() -> Weight {
		(69_569_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_feed_creator() -> Weight {
		(67_570_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
