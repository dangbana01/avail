use anyhow::Result;
use avail_subxt::{
	api, api::runtime_types::bounded_collections::bounded_vec::BoundedVec, avail, build_client,
	tx_send_in_block, AvailConfig, Opts,
};
use structopt::StructOpt;
use subxt::{ext::sp_core::Pair, tx::PairSigner};

/// This example demonstrates creation of application key.
const ALICE_SEED: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	// Account
	let alice = avail::Pair::from_string_with_seed(ALICE_SEED, None).unwrap();
	let signer = PairSigner::<AvailConfig, avail::Pair>::new(alice.0);

	let call = api::tx()
		.data_availability()
		.create_application_key(BoundedVec(b"my_app_name".to_vec()));

	let block = tx_send_in_block!(&client, &call, &signer).block_hash();

	println!("Application key created, block hash: {block:?}");
	Ok(())
}
