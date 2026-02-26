# Crowdfunding DCDT - Using REWA

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let drtjs = await require('@terradharitri/drtjs');
let { drtSys, Rewa, wallets: { alice, bob, carol }} = await drtjs.setupInteractive("local-testnet");

let crowdfunding = await drtSys.loadWrapper("contracts/examples/crowdfunding-dcdt");

// Set the deadline to 1 minute from now (adjust this if you want more time before claiming the rewards)
let someTimeFromNow = await drtSys.currentNonce() + drtjs.minutesToNonce(1);

// Deploy the crowdfunding contract with a target of 2 REWA
await crowdfunding.sender(alice).gas(50_000_000).call.deploy(Rewa(2), someTimeFromNow, Rewa);

// Bob and carol contribute 1.5 REWA each
await crowdfunding.sender(bob).gas(10_000_000).value(Rewa(1.5)).call.fund();
await crowdfunding.sender(carol).value(Rewa(1.5)).call.fund();

// Get the current funds. Note the usage of Rewa.raw (since the balance comes as an integer from the smart contract)
let currentFunds = Rewa.raw(await crowdfunding.query.currentFunds());

// Should print 3 REWA (since bob and carol added 1.5 REWA each)
drtjs.print(currentFunds);

// Confirming the target is 2 REWA
drtjs.print(Rewa.raw(await crowdfunding.query.get_target()));

// Check that alice is the owner
alice.address.equals(await crowdfunding.query.get_owner());

// Store alice's current balance (we'll use this to check the balance difference later on)
let aliceBalanceBefore = await drtSys.getBalance(alice, Rewa);
drtjs.print(aliceBalanceBefore);

// Wait a minute first, otherwise you'll get the "cannot claim before deadline" error
// If the claim doesn't return an error - there are two possibilities:
// - the funding failed, and 1.5 REWA are sent back to both bob and carol
// - it was succesful and alice receives 3 REWA
// Because the target sum specified on deployment was 2 REWA, and we have 3 REWA, the funding should be succesful
await crowdfunding.sender(alice).call.claim();

// Let's check if alice received the funds
let aliceBalanceAfter = await drtSys.getBalance(alice, Rewa);
drtjs.print(aliceBalanceAfter);

// If the previous claim was successful, this prints 2.99 REWA (because of the gas costs)
drtjs.print(aliceBalanceAfter.minus(aliceBalanceBefore));
```
