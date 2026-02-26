# Ping-pong

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let drtjs = await require('@terradharitri/drtjs');
let { drtSys, Rewa, wallets: { alice, bob, carol, dan } } = await drtjs.setupInteractive("local-testnet");

let pingPong = await drtSys.loadWrapper("contracts/examples/ping-pong-rewa");

await pingPong.sender(alice).gas(150_000_000).call.deploy(Rewa(0.5), 2 * 60, null, Rewa(1.5));

await pingPong.gas(20_000_000).sender(alice).value(Rewa(0.5)).ping("note 1");

await pingPong.sender(bob).value(Rewa(0.5)).ping(null);
await pingPong.sender(carol).value(Rewa(0.5)).ping(null);

// this fails because of the balance limit of 1.5 rewa
await pingPong.sender(dan).value(Rewa(0.5).ping(null);

await pingPong.pongAll();

```
