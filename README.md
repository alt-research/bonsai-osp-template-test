# Osp Proof One Step Executor Base on Bonsai Foundry Template

## Local test

start a eth testnet

```bash
anvil
```

Deploy the `BonsaiRelay` contract by running:

```bash
forge script scripts/Deploy.s.sol:Relay --rpc-url http://localhost:8545 --broadcast
```

If use local prove, Start the Bonsai Ethereum Relay by running:

```bash
RELAY_ADDRESS=0x5FbDB2315678afecb367f032d93F642f64180aa3 BONSAI_API_URL=http://localhost:8081 BONSAI_API_KEY=none ./target/release/bonsai-ethereum-relay-cli run 
```

Deploy contract:

```bash
RELAY_ADDRESS=0x5FbDB2315678afecb367f032d93F642f64180aa3 BONSAI_API_URL=http://localhost:8081 BONSAI_API_KEY=none METHOD_NAME=OSPPROOF forge script scripts/Deploy.s.sol:Starter --rpc-url http://localhost:8545 --broadcast
```

Send a transaction to the starter contract:

```bash
cast send --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d --gas-limit 100000 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 'executeOneStep(bytes)' 0x000000023b54220bc7844b9fffa627d96f3ff196bcdd6527b8e8353abfe4375e6cd92a0c0d0000000000000008000000000000000d0000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000a0000006a00142100040000000000000000000000000000000000000000000000000000000000281e78ae4f7026ad42969881adfa0c0d50db8c20ddb8532efbe2dd1bf7ab62cd7136d3de7931c26ca696bb77668d66559d94ff38f85974d15789fec6c34f6828f08c9c082b020547f0f36975a41d8f791f11a933ec454d10bccb33183521329c202f7397e92f15a8ec45453de7ce05f78fba0d831a61dafcd1cabac8881b1e6100
```

Note the proof will transfer state from `0x9b14a1ab2325170896ce70a985ec7a9d2c54bcfdb601d5b20070bc7070f9f70f` to `0x6b5bc5b43331c950fb9dd63a6dacbc7d4070ad514296d833d95538986adeceb9`

Get resp:

```bash
cast call 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 'getPostState(bytes32)' 0x9b14a1ab2325170896ce70a985ec7a9d2c54bcfdb601d5b20070bc7070f9f70f
```

will return `0x6b5bc5b43331c950fb9dd63a6dacbc7d4070ad514296d833d95538986adeceb9`

## Bonsai test

start a eth testnet

```bash
anvil
```

Deploy the `BonsaiRelay` contract by running:

```bash
forge script scripts/Deploy.s.sol:Relay --rpc-url http://localhost:8545 --broadcast
```

Start the Bonsai Ethereum Relay by running:

```bash
RELAY_ADDRESS=0x5FbDB2315678afecb367f032d93F642f64180aa3 BONSAI_API_URL=https://api.bonsai.xyz BONSAI_API_KEY={key} ./target/release/bonsai-ethereum-relay-cli run 
```

Deploy contract:

```bash
RELAY_ADDRESS=0x5FbDB2315678afecb367f032d93F642f64180aa3 BONSAI_API_URL=https://api.bonsai.xyz BONSAI_API_KEY={key} METHOD_NAME=OSPPROOF forge script scripts/Deploy.s.sol:Starter --rpc-url http://localhost:8545 --broadcast
```

Send a transaction to the starter contract:

```bash
cast send --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d --gas-limit 100000 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 'executeOneStep(bytes)' 0x000000023b54220bc7844b9fffa627d96f3ff196bcdd6527b8e8353abfe4375e6cd92a0c0d0000000000000008000000000000000d0000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000a0000006a00142100040000000000000000000000000000000000000000000000000000000000281e78ae4f7026ad42969881adfa0c0d50db8c20ddb8532efbe2dd1bf7ab62cd7136d3de7931c26ca696bb77668d66559d94ff38f85974d15789fec6c34f6828f08c9c082b020547f0f36975a41d8f791f11a933ec454d10bccb33183521329c202f7397e92f15a8ec45453de7ce05f78fba0d831a61dafcd1cabac8881b1e6100
```

Note the proof will transfer state from `0x9b14a1ab2325170896ce70a985ec7a9d2c54bcfdb601d5b20070bc7070f9f70f` to `0x6b5bc5b43331c950fb9dd63a6dacbc7d4070ad514296d833d95538986adeceb9`

Get resp:

```bash
cast call 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 'getPostState(bytes32)' 0x9b14a1ab2325170896ce70a985ec7a9d2c54bcfdb601d5b20070bc7070f9f70f
```

will return `0x6b5bc5b43331c950fb9dd63a6dacbc7d4070ad514296d833d95538986adeceb9`

## Run host for test

```bash
/target/release/host -s 100
```
