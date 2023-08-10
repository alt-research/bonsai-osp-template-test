use methods::{OSP_PROOF_ELF, OSP_PROOF_ID};
use clap::Parser;
use ethabi::{ethereum_types::{H256, U256}, ParamType, Token};
use risc0_zkvm::{default_prover, ExecutorEnv};

mod osp;
mod raw;
use osp::*;

/// Args for prove
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// frontier
    #[arg(short, long)]
    n: u64,
}

fn main() {
    let _ = env_logger::Builder::from_default_env()
        .format_module_path(true)
        .format_level(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    let _args = Args::parse();

    let env = create_env().expect("create env failed");

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, OSP_PROOF_ELF).unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(OSP_PROOF_ID).unwrap();

    log::trace!("receipt {:?}", receipt);

    // let output = ethabi::decode_whole(
    //    &[ParamType::Uint(256), ParamType::Uint(256)],
    //    &receipt.journal,
    //)
    //.expect("decode journal failed");

    let post_hash = H256::from_slice(&receipt.journal);

    log::info!("journal result: {:?}", post_hash);
}
