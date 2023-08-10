use anyhow::Result;
use codec::Encode;
use impl_serde::serialize as bytes;
use ethabi::ethereum_types::H256;
use risc0_zkvm::{
    serde::to_vec, ExecutorEnv, ExitCode, ReceiptMetadata,
};
use serde::Serialize;
use wasmi::{
    core::Value,
    merkle::{DefaultMemoryConfig, MerkleKeccak256},
    proof::{CodeProof, OspProof},
    AsContextMut, Engine, Error, Extern, Instance, Linker, Module, StepResult, Store,
};
use wat::parse_str;

use crate::raw::FIB;

pub type EthConfig = DefaultMemoryConfig<MerkleKeccak256>;

fn setup_module<T>(store: &mut Store<T>, wat: impl AsRef<str>) -> Result<Module, Error> {
    let wasm = parse_str(wat).expect("Illegal wat");
    Module::new(store.engine(), &wasm[..])
}

fn instantiate<T>(store: &mut Store<T>, module: &Module) -> Result<Instance, Error> {
    let linker = <Linker<T>>::new();
    let pre = linker.instantiate(store.as_context_mut(), module)?;
    let instance = pre.ensure_no_start(store.as_context_mut())?;
    Ok(instance)
}

fn call_step<T>(
    store: &mut Store<T>,
    instance: Instance,
    name: &str,
    inputs: &[Value],
    outputs: &mut [Value],
    n: Option<&mut u64>,
) -> Result<StepResult<()>, Error> {
    let f = instance
        .get_export(store.as_context_mut(), name)
        .and_then(Extern::into_func)
        .expect("Could find export function");

    f.step_call(store.as_context_mut(), inputs, outputs, n)
}

fn gen_osp_proof_for_test() -> Result<(OspProof<EthConfig>, CodeProof<MerkleKeccak256>)> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = setup_module(&mut store, FIB).unwrap();
    let instance = instantiate(&mut store, &module).unwrap();

    let code_merkle = store
        .code_proof::<MerkleKeccak256>(instance)
        .make_code_merkle();

    let code_proof = code_merkle.code_proof();

    let inputs = vec![Value::I32(10)];
    let mut outputs = vec![Value::I32(0)];

    // change steps to test differnt inst.
    let mut steps = 10;
    let res = call_step(
        &mut store,
        instance,
        "fib",
        &inputs,
        &mut outputs,
        Some(&mut steps),
    )?;

    let pc = match res {
        StepResult::Results(()) => unreachable!(),
        StepResult::RunOutOfStep(pc) => pc,
    };

    let osp_proof = store
        .osp_proof::<DefaultMemoryConfig<MerkleKeccak256>>(&code_merkle, instance)
        .make_osp_proof_v0(pc)?;

    log::info!("osp inst: {:?}", osp_proof.inst_proof.inst);

    Ok((osp_proof, code_proof))
}

pub fn create_env<'a>() -> Result<ExecutorEnv<'a>> {
    let (mut osp_proof, code_proof) = gen_osp_proof_for_test()?;

    let osp_proof_bytes = osp_proof.encode();
    let code_proof_bytes = code_proof.encode();

    log::info!(
        "env: osp_proof len {}, code_proof len {}",
        osp_proof_bytes.len(),
        code_proof_bytes.len()
    );

    osp_proof.run(&code_proof)?;
    let post_proof_hash = osp_proof.hash();

    log::info!("post_proof_hash: {:?}", H256::from(post_proof_hash));

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&osp_proof_bytes).unwrap())
        .add_input(&to_vec(&code_proof_bytes).unwrap())
        .add_input(&to_vec(&post_proof_hash.to_vec()).unwrap())
        .build()?;

    Ok(env)
}
