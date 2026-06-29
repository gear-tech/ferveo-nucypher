# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (2026-06-29)

<csr-id-379dabea4ab9c9e428a69c6683eafe2447ebf788/>
<csr-id-ea186a8b45fdf3e647f973b5523626af1ae001e8/>
<csr-id-50a9861635787aef175d1fa96c91773920ae62f1/>
<csr-id-c70d2605882a9314abcc075fa6df117892ea532e/>
<csr-id-1f00d41e5aa7aa536b956fc5863a0462a9b7e188/>
<csr-id-18d49bcb39ede98b80873e2b26b2e3b4ba0401e0/>
<csr-id-be89332821d2fa301519787075e5454963271087/>
<csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/>
<csr-id-802e7121d5eb5a31617bf88c4e14fe79d45e68e3/>
<csr-id-58002f50155df31a11b9d58d94750a2ed1076102/>
<csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/>
<csr-id-0ef7de4c9b4442e2c6125d457de9420146be50b7/>
<csr-id-8b26396cc26ceeddca52dc37ac9461f0bb93ecfe/>
<csr-id-4a8375d1873560241ae8eea96230a42635ed1764/>
<csr-id-6e3369d11cfd4ec751775e1eee82f8192b51943e/>
<csr-id-315d2b4cc2825e13820d9c64639490c44b538385/>

### Chore

 - <csr-id-379dabea4ab9c9e428a69c6683eafe2447ebf788/> prepare crates to publish
   * start preparing for publishing
   
   * apply new formatting style
   
   * prepare to publish
 - <csr-id-ea186a8b45fdf3e647f973b5523626af1ae001e8/> fix serialize/deserialize with serde-hex
 - <csr-id-50a9861635787aef175d1fa96c91773920ae62f1/> fix +nightly fmt
 - <csr-id-c70d2605882a9314abcc075fa6df117892ea532e/> make logic prettier
 - <csr-id-1f00d41e5aa7aa536b956fc5863a0462a9b7e188/> serialize/deserialize for G1, G2, GT points
 - <csr-id-18d49bcb39ede98b80873e2b26b2e3b4ba0401e0/> dirty implementation Encode/Decode for Ciphertext
 - <csr-id-be89332821d2fa301519787075e5454963271087/> bump arkworks to 0.6, edition 2024, remove python/wasm bindings
   Modernize the workspace for the gear-tech fork:
   
   - Bump ark-bls12-381, ark-ec, ark-ff, ark-poly, ark-serialize, ark-std
     from 0.4 to 0.6. Fix breaking API changes:
     - Group trait split: rename to PrimeGroup in pvss.rs, refresh.rs,
       decryption.rs.
     - FixedBase removed: rewrite subproductdomain::fast_multiexp on top
       of CurveGroup::batch_mul.
     - Pairing::pairing requires Into<G1Prepared>: deref &G1 in pvss.rs.
   
   - Replace miracl_core hash-to-curve with native ark_ec::hashing.
     Drops a heavy C-port crypto dep and the byte-order/flag workarounds.
     RFC9380 J.10.1 test vectors still pass.
   
   - Bump edition 2021 to 2024 across all crates, set resolver = "3".
   
   - Drop ferveo-python and ferveo-wasm crates entirely along with
     bindings_python.rs / bindings_wasm.rs inside ferveo. Removes
     pyo3 0.18 and wasm-bindgen-derive 0.2 baggage; gear-tech only
     consumes the native crates.
   
   - Replace assert_ne!(x, x) clippy::eq_op placeholders inside ignored
     recovery tests (#193) with explicit panic!.
   
   - rust-toolchain: 1.87.0 -> stable, drop wasm32 target.
   - CI workflow: drop 1.87 MSRV matrix and wasm-test job; use nightly
     rustfmt (rustfmt.toml uses unstable imports_granularity).
   
   cargo build/test/clippy/fmt all green on workspace --all-features.
   93 tests passing, 11 ignored (upstream pre-existing #193).
 - <csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/> move shared dependencies to workspace crate
 - <csr-id-802e7121d5eb5a31617bf88c4e14fe79d45e68e3/> remove duplicated field
 - <csr-id-58002f50155df31a11b9d58d94750a2ed1076102/> rename ferveo-tpke package to ferveo-tdec

### Chore

 - <csr-id-979a12c0a4600283ffdf9aafcdb97d2126814596/> serde Serialize/Deserialize for generic types over E: Pairing
   * update serialize/deserialize paths
   
   * serde bound for types generic over E: Pairing

### New Features

 - <csr-id-cb0750a69560a21ceab8c7a55ead3d1ff8079161/> implement dealer
 - <csr-id-1ad011b6789eda16c52ca23600f7533be47584e6/> implement missing parity_scale_codec
   * parity_scale_codec for DecryptionShareSimple
   
   * chore: downgrade arkworks dependencies
   
   * re-export serialization modules
   
   * remove useless Vec<> input param
   
   * remove useless Result<>
   
   * chore: fix clippy for benches
   
   * chore: implement Encode + Decode for SharedSecret
   
   * feat: implement encode/decode helpers for parity_scale_codec
   
   * chore: add public decryption context alias
   
   * chore: DecryptionContext compatible with clap
   
   * update documentation for methods

### Bug Fixes

 - <csr-id-975dae0d5f8d1a2e5c061fbc8d11b1cc73c867d7/> not using subset of participants in precomputed variant

### Other

 - <csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/> introduce refreshing api in ferveo

### Refactor

 - <csr-id-0ef7de4c9b4442e2c6125d457de9420146be50b7/> rename public key share to public key
 - <csr-id-8b26396cc26ceeddca52dc37ac9461f0bb93ecfe/> avoid using crypto primitives directly, part 1

### Test

 - <csr-id-4a8375d1873560241ae8eea96230a42635ed1764/> fix tests sensitive to message ordering

### Other (BREAKING)

 - <csr-id-6e3369d11cfd4ec751775e1eee82f8192b51943e/> remove fast variant
 - <csr-id-315d2b4cc2825e13820d9c64639490c44b538385/> remove state from dkg, part 1

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#10](https://github.com/gear-tech/ferveo-nucypher/issues/10), [#9](https://github.com/gear-tech/ferveo-nucypher/issues/9)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/gear-tech/ferveo-nucypher/issues/10)**
    - Implement missing parity_scale_codec ([`1ad011b`](https://github.com/gear-tech/ferveo-nucypher/commit/1ad011b6789eda16c52ca23600f7533be47584e6))
 * **[#9](https://github.com/gear-tech/ferveo-nucypher/issues/9)**
    - Serde Serialize/Deserialize for generic types over E: Pairing ([`979a12c`](https://github.com/gear-tech/ferveo-nucypher/commit/979a12c0a4600283ffdf9aafcdb97d2126814596))
</details>

## 0.4.0 (2025-08-15)

<csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/>
<csr-id-802e7121d5eb5a31617bf88c4e14fe79d45e68e3/>
<csr-id-58002f50155df31a11b9d58d94750a2ed1076102/>
<csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/>
<csr-id-0ef7de4c9b4442e2c6125d457de9420146be50b7/>
<csr-id-8b26396cc26ceeddca52dc37ac9461f0bb93ecfe/>
<csr-id-4a8375d1873560241ae8eea96230a42635ed1764/>
<csr-id-6e3369d11cfd4ec751775e1eee82f8192b51943e/>
<csr-id-315d2b4cc2825e13820d9c64639490c44b538385/>

### Chore

 - <csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/> move shared dependencies to workspace crate
 - <csr-id-802e7121d5eb5a31617bf88c4e14fe79d45e68e3/> remove duplicated field
 - <csr-id-58002f50155df31a11b9d58d94750a2ed1076102/> rename ferveo-tpke package to ferveo-tdec

### Bug Fixes

 - <csr-id-975dae0d5f8d1a2e5c061fbc8d11b1cc73c867d7/> not using subset of participants in precomputed variant

### Other

 - <csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/> introduce refreshing api in ferveo

### Refactor

 - <csr-id-0ef7de4c9b4442e2c6125d457de9420146be50b7/> rename public key share to public key
 - <csr-id-8b26396cc26ceeddca52dc37ac9461f0bb93ecfe/> avoid using crypto primitives directly, part 1

### Test

 - <csr-id-4a8375d1873560241ae8eea96230a42635ed1764/> fix tests sensitive to message ordering

### Other (BREAKING)

 - <csr-id-6e3369d11cfd4ec751775e1eee82f8192b51943e/> remove fast variant
 - <csr-id-315d2b4cc2825e13820d9c64639490c44b538385/> remove state from dkg, part 1

## 0.2.0 (2023-08-28)

### New Features (BREAKING)

 - <csr-id-1800d3c5db164947c7cae35433fb8e3ad2650b66/> add ciphertext header to ciphertext api

## v0.1.0 (2023-07-07)

<csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/>

### Chore

 - <csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/> adjust changelogs for cargo-smart-release

