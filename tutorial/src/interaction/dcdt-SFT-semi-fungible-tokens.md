# Semi-Fungible Tokens (SFTs).

Semi-Fungible Tokens have variable amounts, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let drtjs = await require('@numbatnetwork/drtjs');
let { drtSys, Rewa, wallets: { alice, bob, carol } } = await drtjs.setupInteractive("local-testnet");

// Issue a new semi-fungible token
let MyToken = await drtSys.sender(alice).issueSemiFungible("MySemiFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await drtSys.dcdtSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "DCDTRoleNFTCreate", "DCDTRoleNFTAddQuantity");

// Create a new nonce
let MyFirstSemi = await drtSys.sender(alice).dcdtNftCreate(MyToken, 1_000, "MyFirstSemi", 0, "", "", "https://example.com");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await drtSys.getBalance(alice, MyFirstSemi).then(drtjs.print);

// Send some tokens to bob and carol
await drtSys.sender(alice).value(MyFirstSemi(200)).send(bob);

await drtSys.getBalance(alice, MyFirstSemi).then(drtjs.print);
await drtSys.getBalance(bob, MyFirstSemi).then(drtjs.print);

```
