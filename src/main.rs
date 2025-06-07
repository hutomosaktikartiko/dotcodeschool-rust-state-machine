mod balances;
mod system;

/// This it out main Runtime.
/// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<&'static str, u32, u32>,
	balances: balances::Pallet<&'static str, u128>,
}

impl Runtime {
	/// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	// create a mutable variable called `runtime`
	let mut runtime = Runtime::new();

	// set the balance of `alice` to 100
	runtime.balances.set_balance(&"alice", 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&"alice");
	let _res = runtime.balances.transfer("alice", "bob", 30).map_err(|e| eprintln!("{}", e));

	// second transaction
	runtime.system.inc_nonce(&"alice");
	let _res = runtime.balances.transfer("alice", "charlie", 30).map_err(|e| eprint!("{}", e));

	println!("{:#?}", runtime);
}
