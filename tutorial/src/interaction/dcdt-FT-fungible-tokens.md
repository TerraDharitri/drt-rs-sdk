# Fungible Tokens (FT)

Fungible Tokens have variable amounts, but always have nonce 0. They may be denominated.

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let drtjs = await require('@numbatnetwork/drtjs');
let { drtSys, wallets: { alice, bob, carol } } = await drtjs.setupInteractive("local-testnet");

// Issue a new fungible token
let MyToken = await drtSys.sender(alice).issueFungible("MyFungibleToken", "MYTOKEN", 1_000_00, 2);

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

// Note: if you have the token identifier, you can recall the token via:
// let MyToken = await drtSys.recallToken("MYTOKEN-a4fc62");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await drtSys.getBalance(alice, MyToken).then(drtjs.print);

// Send some tokens to bob
await drtSys.sender(alice).value(MyToken(200.0)).send(bob);

// Check alice's balance (should be 800.00 MYTOKEN)
await drtSys.getBalance(alice, MyToken).then(drtjs.print);

// Check bob's balance (should be 200.00 MYTOKEN)
await drtSys.getBalance(bob, MyToken).then(drtjs.print);

```
