# <h1 align="center"> substrate-simnode ⚙️ </h1>


This library exists to allow substrate runtime developers:
 - ✅ Test complex pallets that require a full runtime, not a mocked runtime.
 - ✅ Simulate runtime upgrades.
 - ✅ Simulate runtime migrations.
 - ✅ Simulate transactions on their live chain state.

This library meets its stated goals by setting up a full substrate node complete with all subsystems (networking, transaction pool, runtime executor, import_queue, rpc, block authorship via manual seal).

Since its a full node environment, <b>it also supports resuming an existing live chain state, given the path to one</b>. 

***

## Examples

```rust
struct YourRuntimeChainInfo;
impl substrate_simnode::ChainInfo for YourRuntimeChainInfo {
    // fill in implementation details
}

fn main() {
    substrate_simnode::parachain_node::<YourRuntimeChainInfo, _>(|node| async move {
        // node is a handle to the running subsystems, use it to:
        // - submit transactions to the transaction pool, from any on-chain account
        // - seal blocks
        // - inspect pallet state
        // - revert blocks
    });
}

```


Check out the [examples folder](./examples) to see how to set up simnode for your substrate runtime.

## Tutorials

[Testing parachain runtimes with Simnode | Substrate Seminar](https://www.youtube.com/watch?v=0FvcABti7yk)

