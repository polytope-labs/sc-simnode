## substrate-simnode ⚙️

This library exists to allow substrate runtime developers test:
 - [x] complex pallets that require a full runtime
 - [x] runtime upgrades
 - [x] runtime migrations

This library meets its stated goals by setting up a full node complete with all subsystems (networking, import_queue, rpc, block authorship via manual seal).
Since its a full node environment, <b>it also supports resuming an existing live chain state, given the path to one</b>. 

***


## How does it work?

First you have to describe the chain you're trying to simulate.

```rust
struct YourRuntimeInfo;

impl ChainInfo for YourRuntimeInfo {
    // fill in the neccessary details
}

// next create the neccessary client subsystems
let node = build_node_subsystems::<NodeTemplateChainInfo, _>(
    ConfigOrChainSpec::ChainSpec(
        Box::new(development_config()),
        tokio_runtime.handle().clone(),
    ),
    |_client, _select_chain, _keystore| {
        // chain specific block_import, consensus_data_provider and inherent_data_provider
    }
);

// create blocks, empty if no txs in in the pool
node.seal_blocks(10).await;

// submit extrinsics into the node's txpool.
```