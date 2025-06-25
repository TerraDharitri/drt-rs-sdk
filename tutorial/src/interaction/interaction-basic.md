# Preparations

# Setting up the local testnet

The following examples rely on having a [local testnet](https://docs.dharitri.org/developers/setup-local-testnet/) up and running.

# Installing @terradharitri/drtjs globally

```bash
cd ./code/numbat-sdk-drtjs
npm run compile && npm test && npm install -g
```

# How to start a node terminal

By exporting `NODE_PATH`, the node terminal should have access to `drtjs`.
Open a terminal and enter the following:

```bash
cd ./code/numbat-wasm-rs
export NODE_PATH=$HOME/.nvm/versions/node/$(node --version)/lib/node_modules
node --experimental-repl-await
```

# Basic DCDT usage

- [Fungible Tokens (FTs)](dcdt-FT-fungible-tokens.md)
- [Semi-Fungible Tokens (SFTs)](dcdt-SFT-semi-fungible-tokens.md)
- [Non-Fungible Tokens (NFTs)](dcdt-NFT-non-fungible-tokens.md)

# Smart contract examples

- Adder [interaction](../../../contracts/examples/adder/interaction/Adder.drtjs.md)
- Crowdfunding DCDT [REWA interaction](../../../contracts/examples/crowdfunding-dcdt/interaction/Crowdfunding-rewa.drtjs.md), [DCDT interaction](../../../contracts/examples/crowdfunding-dcdt/interaction/Crowdfunding-dcdt.drtjs.md)
- Multisig [REWA adder interaction](../../../contracts/examples/multisig/interaction/Multisig-adder-rewa.drtjs.md)
- Ping-pong [REWA interaction](../../../contracts/examples/ping-pong-rewa/interaction/Ping-pong-rewa.drtjs.md)
