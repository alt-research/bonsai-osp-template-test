use methods::{FIBONACCI_ELF, FIBONACCI_ID};
use clap::Parser;
use ethabi::{ethereum_types::U256, ParamType, Token};
use risc0_zkvm::{default_prover, ExecutorEnv};

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

    let args = Args::parse();

    // Encode the arguments
    let input = ethabi::encode(&[Token::Uint(U256::from(args.n))]);

    // First, we construct an executor environment
    let env = ExecutorEnv::builder().add_input(&input).build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, FIBONACCI_ELF).unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(FIBONACCI_ID).unwrap();

    log::trace!("receipt {:?}", receipt);

    let output = ethabi::decode_whole(
        &[ParamType::Uint(256), ParamType::Uint(256)],
        &receipt.journal,
    )
    .expect("decode journal failed");

    log::info!("journal result: {:?}", output);
}
