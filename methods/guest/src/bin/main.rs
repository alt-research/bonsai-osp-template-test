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

use codec::Decode;
use ethabi::{ethereum_types::{H256, U256}, ParamType, Token};
use risc0_zkvm::guest::env;
use wasmi::{
    merkle::{DefaultMemoryConfig, MerkleKeccak256},
    proof::{CodeProof, OspProof},
};

risc0_zkvm::guest::entry!(main);

pub type EthConfig = DefaultMemoryConfig<MerkleKeccak256>;

fn main() {
    // Read data sent from the application contract.
    let osp_proof_bytes: Vec<u8> = env::read();
    let code_proof_bytes: Vec<u8> = env::read();
    let post_proof_hash: Vec<u8> = env::read();

    // run osp proof
    let mut osp_proof: OspProof<EthConfig> =
        Decode::decode(&mut &*osp_proof_bytes).expect("osp proof");
    let code_proof: CodeProof<MerkleKeccak256> =
        Decode::decode(&mut &*code_proof_bytes).expect("code proof");

    osp_proof.run(&code_proof).expect("osp proof run");

    // the proof hash can check if the proof is correct.
    let proof_hash = osp_proof.hash().to_vec();

    // should be eq
    assert_eq!(proof_hash, post_proof_hash);

    // Commit the journal that will be received by the application contract.
    // Encoded types should match the args expected by the application callback.
    env::commit_slice(&proof_hash);
}
