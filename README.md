<h2 align="center">
  Vector Database on ICP
</h2>

Welcome to our new vector_database project and to the internet computer development community. 

# What is ELNA's VectorDB

It is an open-source and fully on-chain vector database and vector similarity search engine, that helps AI applications on ICP. 

This is released under the [open-source Apache License 2.0] in October 2024.


# Instant Distance: fast HNSW indexing
Here we using HNSW algorthm to index the vecotr embeddings. [Instance Distance](https://github.com/instant-labs/instant-distance) is a fast pure-Rust implementation of the Hierarchical Navigable Small Worlds paper by Malkov and Yashunin for finding approximate nearest neighbors (ANN).

# VectorDB Features

## Storage Capabilities
- **Variable Dimension Storage**: Our VectorDB supports the storage of vectors with varying dimensions, allowing for flexible data management.

## Stability and Persistence
- **Stable Memory Support**: VectorDB ensures data persistence across upgrades. The data is stored in stable memory during the pre-upgrade hook and is reloaded into the heap after the canister upgrade, maintaining data integrity and continuity.

## Security and Access Control
- **Super User and Admin Management**: 

  - **Super User**: Each vector canister has a designated super user who has full control over the canister.

  - **Admin Management**: The super user can add or remove admin users who have permissions to read or write in the VectorDB.
  
  - **Stable Structure Support**: The security structure is also stable, ensuring that the access control mechanisms persist through upgrades.

---


# Getting Started
To learn more before you start working with vector_database, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)


If you want to start working on your project right away, you might want to try the following commands:

```bash
cd vector_database/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

