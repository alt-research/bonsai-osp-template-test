use bonsai_starter_methods::{OSP_PROOF_ELF, OSP_PROOF_ID};
use clap::Parser;
use ethabi::{ethereum_types::H256, ParamType};
use risc0_zkvm::default_executor_from_elf;

mod osp;
mod raw;
use osp::*;

/// Args for prove
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the inst step for executor osp proof
    #[arg(short, long)]
    step: u64,

    /// if show debug log
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let log_level = if args.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let _ = env_logger::Builder::from_default_env()
        .format_module_path(true)
        .format_level(true)
        .filter_level(log_level)
        .try_init();

    let env = create_env(args.step).expect("create env failed");

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, OSP_PROOF_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(OSP_PROOF_ID).unwrap();

    log::trace!("receipt {:?}", receipt);

    // function storeResult(bytes32 preState, bytes32 postState)
    let output = ethabi::decode_whole(
        &[ParamType::FixedBytes(32), ParamType::FixedBytes(32)],
        &receipt.journal,
    )
    .expect("decode journal failed");

    let per_hash = H256::from_slice(
        &output[0]
            .clone()
            .into_fixed_bytes()
            .expect("decode pre-hash failed"),
    );
    let post_hash = H256::from_slice(
        &output[1]
            .clone()
            .into_fixed_bytes()
            .expect("decode post-hash failed"),
    );

    log::info!("journal result: {:?} -> {:?}", per_hash, post_hash);
}
