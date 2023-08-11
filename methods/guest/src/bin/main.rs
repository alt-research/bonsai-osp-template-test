// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use std::io::Read;

use codec::Decode;
use ethabi::{ParamType, Token};
use risc0_zkvm::guest::env;
use wasmi::{
    merkle::{DefaultMemoryConfig, MerkleKeccak256},
    proof::{CodeProof, OspProof},
};

risc0_zkvm::guest::entry!(main);

pub type EthConfig = DefaultMemoryConfig<MerkleKeccak256>;

fn main() {
    // Read data sent from the application contract.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();

    // Type array passed to `ethabi::decode_whole` should match the types encoded in
    // the application contract.
    // abi.encode(instRoot, funcRoot, proof)
    let input = ethabi::decode(
        &[
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::Bytes,
        ],
        &input_bytes,
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

    // run osp proof
    let mut osp_proof: OspProof<EthConfig> =
        Decode::decode(&mut &*osp_proof_bytes).expect("osp proof");
    let code_proof = CodeProof::<MerkleKeccak256> {
        func_root,
        inst_root,
    };

    let pre_root = osp_proof.hash().to_vec();

    osp_proof.run(&code_proof).expect("osp proof run");

    // the proof hash can check if the proof is correct.
    let proof_hash = osp_proof.hash().to_vec();

    // Commit the journal that will be received by the application contract.
    // Encoded types should match the args expected by the application callback.
    env::commit_slice(&ethabi::encode(&[
        Token::FixedBytes(pre_root),
        Token::FixedBytes(proof_hash),
    ]));
}
