use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

/// This is the System Pallet
/// It handled low level state need for you blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce
	nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + One + AddAssign + Copy,
	Nonce: Zero + One + AddAssign + Copy,
{
	/// Create a new instance of the system Pallet
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	/// This function can be used to increment the block number.
	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number.add_assign(BlockNumber::one());
	}

	/// Increment the nonce of an account.
	/// This helps us keep track of how many transactions each account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
		*self.nonce.entry(who.clone()).or_insert(Nonce::zero()) += Nonce::one();
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<&'static str, u32, u32>::new();

		system.inc_block_number();
		system.inc_nonce(&"alice");

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
