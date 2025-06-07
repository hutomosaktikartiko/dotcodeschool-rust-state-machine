use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// This it the balance Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<AccountID, Balance> {
	/// A simple storage mapping from accounts (`String`) to thier balances (`u128`).
	balances: BTreeMap<AccountID, Balance>,
}

impl<AccountID, Balance> Pallet<AccountID, Balance>
where
	AccountID: Ord + Clone,
	Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
	/// Create a new instance of the balances module
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`
	pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountID) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	/// Transfer `amount` from one account to another.
	/// This functions verifies that `from` has  at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: AccountID,
		to: AccountID,
		amount: Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow occured")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balance() {
		let mut balances = super::Pallet::<&'static str, u128>::new();

		assert_eq!(balances.balance(&"alice"), 0);
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.balance(&"alice"), 100);
		assert_eq!(balances.balance(&"bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<&'static str, u128>::new();

		assert_eq!(balances.transfer("alice", "bob", 10), Err("Not enough funds"));
		balances.set_balance(&"alice", 100);
		assert_eq!(balances.transfer("alice", "bob", 10), Ok(()));
		assert_eq!(balances.balance(&"alice"), 90);
		assert_eq!(balances.balance(&"bob"), 10);
	}
}
