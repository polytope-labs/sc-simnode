# sc-simnode ![Unit Tests](https://github.com/polytope-labs/sc-simnode/actions/workflows/ci.yml/badge.svg) [![Telegram: YourGroup](https://img.shields.io/badge/-Telegram-blue?style=flat-square&logo=Telegram&logoColor=white&link=https://t.me/YourGroup)](https://t.me/sc_simnode) [![Discord: YourServer](https://img.shields.io/badge/-Discord-7289DA?style=flat-square&logo=Discord&logoColor=white&link=https://discord.gg/YourServer)](https://discord.gg/2vbPnFwg8h)

![alt text](./assets/neo.webp)

> I'm trying to free your mind, Neo. But I can only show you the door. You're the one that has to walk through it.

<br />

This library exists to allow substrate developers to:
 - [x] Test complex pallets that require a full runtime, not a mocked runtime.
 - [x] Test a combination of both onchain & offchain components (eg offchain workers)
 - [x] Fork your live chain state and execute transactions from any origin.
 - [x] Simulate runtime upgrades & migrations.


## Documentation

Installation and integration guides can be found in the [book](https://simnode.polytope.technology). Example integrations are provided for runtimes of all different kinds:

- [x] [Standalone Chain](/examples/aura), AURA leader election, GRANDPA consensus.
- [X] [Standalone Chain](/examples/babe), BABE leader election, GRANDPA consensus.
- [X] [Parachain](/examples/parachain), AURA leader election, Polkadot Consensus.

## License

This library is licensed under the Apache 2.0 License, Copyright (c) 2023 Polytope Labs.