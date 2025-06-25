# Non-Fungible Tokens (NFTs)

Non-Fungible Tokens have amounts of either 0 or 1, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let drtjs = await require('@numbatnetwork/drtjs');
let { drtSys, Rewa, wallets: { alice, bob, carol } } = await drtjs.setupInteractive("local-testnet");

// Issue a new non-fungible token
let MyToken = await drtSys.sender(alice).issueNonFungible("MyFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await drtSys.dcdtSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "DCDTRoleNFTCreate");

// Create 2 tokens
let MyFirstNFT = await drtSys.sender(alice).dcdtNftCreate(MyToken, 1, "MyFirstNFT", 0, "", "", "https://example.com");
let MySecondNFT = await drtSys.sender(alice).dcdtNftCreate(MyToken, 1, "MySecondNFT", 0, "", "", "https://example.com");

// Send some tokens to bob and carol
await drtSys.sender(alice).value(MyFirstNFT.one()).send(bob);
await drtSys.sender(alice).value(MySecondNFT.one()).send(carol);

await drtSys.getBalanceList(alice, MyToken).then(drtjs.printList);
await drtSys.getBalanceList(bob, MyToken).then(drtjs.printList);
```
