use std::collections::BTreeMap;

/// This is the System Pallet
/// It handled low level state need for you blockchain.
pub struct Pallet {
	/// The current block number.
	block_number: u32,
	/// A map from an account to their nonce
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the system Pallet
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	/// This function can be used to increment the block number.
	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	/// Get the none of an account `who`
	/// If the has no stored nonce, we return zero
	pub fn nonce(&self, who: &String) -> u32 {
		*self.nonce.get(who).unwrap_or(&0)
	}

	/// Increment the nonce of an account.
	/// This helps us keep track of how many transactions each account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		*self.nonce.entry(who.to_string()).or_insert(0) += 1;
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut pallet = super::Pallet::new();

		pallet.inc_block_number();
		pallet.inc_nonce(&"alice".to_string());

		assert_eq!(pallet.block_number(), 1);
		assert_eq!(pallet.nonce(&"alice".to_string()), 1);

		assert_eq!(pallet.nonce(&"bob".to_string()), 0);
	}
}
