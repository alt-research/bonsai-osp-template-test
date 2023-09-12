use anyhow::Result;
use codec::Encode;
use ethabi::{ethereum_types::H256, Token};
use risc0_zkvm::ExecutorEnv;
use wasmi::{
    core::Value,
    merkle::{DefaultMemoryConfig, MerkleKeccak256},
    proof::{CodeProof, OspProof},
    AsContextMut, Engine, Error, Extern, Instance, Linker, Module, StepResult, Store,
};
use wat::parse_str;

use crate::raw::FIB;

pub type EthConfig = DefaultMemoryConfig<MerkleKeccak256>;

#[allow(dead_code)]
fn setup_module_from_wat<T>(store: &mut Store<T>, wat: impl AsRef<str>) -> Result<Module, Error> {
    let wasm = parse_str(wat).expect("Illegal wat");
    Module::new(store.engine(), &wasm[..])
}

#[allow(dead_code)]
fn setup_module_from_wasm<T>(store: &mut Store<T>, code: &[u8]) -> Result<Module, Error> {
    Module::new(store.engine(), code)
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

fn gen_osp_proof_for_test(
    mut steps: u64,
) -> Result<(OspProof<EthConfig>, CodeProof<MerkleKeccak256>)> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = setup_module_from_wat(&mut store, FIB).unwrap();
    let instance = instantiate(&mut store, &module).unwrap();

    let code_merkle = store
        .code_proof::<MerkleKeccak256>(instance)
        .make_code_merkle();

    let code_proof = code_merkle.code_proof();

    let inputs = vec![Value::I32(10)];
    let mut outputs = vec![Value::I32(0)];

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

pub fn create_env<'a>(step: u64) -> Result<ExecutorEnv<'a>> {
    let (mut osp_proof, code_proof) = gen_osp_proof_for_test(step)?;

    // for fib :
    // 0xccc2d8707343c7348538f6d0114fab4e20437ec900592ba0d126fab4e19648fe
    // 0x7080aa6f23c6857049c90bc7103a883b2fbe2f4ab895834a06a87a18d9a60a87
    log::info!("code func_root: 0x{}", hex::encode(code_proof.func_root));
    log::info!("code inst_root: 0x{}", hex::encode(code_proof.inst_root));

    let osp_proof_bytes = osp_proof.encode();

    log::debug!("env: osp_proof len {}", osp_proof_bytes.len(),);

    let pre_proof_hash = osp_proof.hash();
    osp_proof.run(&code_proof)?;
    let post_proof_hash = osp_proof.hash();

    log::info!(
        "executor hash: {:?} -> {:?}",
        H256::from(pre_proof_hash),
        H256::from(post_proof_hash)
    );

    log::info!(
        "code len: {}, {}",
        code_proof.inst_root.to_vec().len(),
        code_proof.func_root.to_vec().len()
    );

    log::info!(
        "in  0x{} 0x{} 0x{}",
        hex::encode(code_proof.func_root),
        hex::encode(code_proof.inst_root),
        hex::encode(osp_proof_bytes.clone())
    );

    // abi.encode(instRoot, funcRoot, proof)
    let data = ethabi::encode(&[
        Token::FixedBytes(code_proof.inst_root.to_vec()),
        Token::FixedBytes(code_proof.func_root.to_vec()),
        Token::Bytes(osp_proof_bytes),
    ]);

    {
        // TODO: should use ethabi::decode_whole, but have some errors
        use ethabi::ParamType;
        let input = ethabi::decode(
            &[
                ParamType::FixedBytes(32),
                ParamType::FixedBytes(32),
                ParamType::Bytes,
            ],
            &data,
        )
        .unwrap();

        let inst_root: [u8; 32] = input[0]
            .clone()
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();
        let func_root: [u8; 32] = input[1]
            .clone()
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();
        let osp_proof_bytes = input[2].clone().into_bytes().unwrap();

        log::info!(
            "res 0x{} 0x{} 0x{}",
            hex::encode(func_root),
            hex::encode(inst_root),
            hex::encode(osp_proof_bytes)
        );
    }

    let env = ExecutorEnv::builder().add_input(&data).build()?;

    Ok(env)
}
