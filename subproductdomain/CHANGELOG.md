# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.5.0 (2026-06-11)

### Chore

 - <csr-id-379dabea4ab9c9e428a69c6683eafe2447ebf788/> prepare crates to publish
   * start preparing for publishing
   
   * apply new formatting style
   
   * prepare to publish
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
 - <csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/> adjust changelogs for cargo-smart-release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#7](https://github.com/gear-tech/ferveo-nucypher/issues/7), [#72](https://github.com/gear-tech/ferveo-nucypher/issues/72)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#7](https://github.com/gear-tech/ferveo-nucypher/issues/7)**
    - Prepare crates to publish ([`379dabe`](https://github.com/gear-tech/ferveo-nucypher/commit/379dabea4ab9c9e428a69c6683eafe2447ebf788))
 * **[#72](https://github.com/gear-tech/ferveo-nucypher/issues/72)**
    - Refactor subproductdomain ([`2d8026b`](https://github.com/gear-tech/ferveo-nucypher/commit/2d8026b2299fd9b67c77fb3b4e565ff9f4e6505b))
 * **Uncategorized**
    - Merge pull request #1 from gear-tech/chore/ark-0.6-and-cleanup ([`4f7f8f9`](https://github.com/gear-tech/ferveo-nucypher/commit/4f7f8f9855aafc19bb93c6dd367b6c057d5b65d0))
    - Bump arkworks to 0.6, edition 2024, remove python/wasm bindings ([`be89332`](https://github.com/gear-tech/ferveo-nucypher/commit/be89332821d2fa301519787075e5454963271087))
    - Release ferveo-nucypher-common v0.4.0, subproductdomain-nucypher v0.4.0, ferveo-nucypher-tdec v0.4.0, ferveo-nucypher v0.4.0 ([`b50d448`](https://github.com/gear-tech/ferveo-nucypher/commit/b50d448521b5fb29c630af4c9a3994f3b40060c8))
    - Merge pull request #188 from nucypher/rocknroll ([`1e66268`](https://github.com/gear-tech/ferveo-nucypher/commit/1e66268dfbfbf76566b4bcf6c25a9852692bb380))
    - Merge pull request #211 from derekpierre/mrkrabs ([`763e06b`](https://github.com/gear-tech/ferveo-nucypher/commit/763e06bb2375e2ded95b409e282ae1f491e16d59))
    - Merge pull request #205 from cygnusv/mrkrabs ([`bb51e96`](https://github.com/gear-tech/ferveo-nucypher/commit/bb51e963f552d2ced387d0ac5c4b311f13715eb4))
    - Update cargo.toml of all ferveo packages for public release. ([`d21ea18`](https://github.com/gear-tech/ferveo-nucypher/commit/d21ea1826f81f47ee88a64dcb98678560e691e57))
    - Update cargo.toml of all ferveo packages for test release. ([`000dc17`](https://github.com/gear-tech/ferveo-nucypher/commit/000dc1715c31f2a32f2366feb6ca652b57d40130))
    - Update wasm-bindgen ([`19e228b`](https://github.com/gear-tech/ferveo-nucypher/commit/19e228b70920b359d93175dfcc5470062832102c))
    - Update cargo.toml of all ferveo packages ([`4e03d43`](https://github.com/gear-tech/ferveo-nucypher/commit/4e03d43255c2fceb729bf2227bff396a25d700c5))
    - Update authors ([`380e984`](https://github.com/gear-tech/ferveo-nucypher/commit/380e9840f0b491da002ff02b863230f5824b500e))
    - Merge pull request #189 from piotr-roslaniec/workspace-deps ([`be98542`](https://github.com/gear-tech/ferveo-nucypher/commit/be9854252fdff297d99a63eb443a473ecfd41f5a))
    - Move shared dependencies to workspace crate ([`983110c`](https://github.com/gear-tech/ferveo-nucypher/commit/983110c4dbb41eb7f0fba2c06f561b68718d0f29))
    - Merge pull request #138 from nucypher/development ([`434fd5d`](https://github.com/gear-tech/ferveo-nucypher/commit/434fd5d07b54e72d120e9aa06cbc3e47848e6bcf))
    - Release ferveo-common-pre-release v0.1.0, subproductdomain-pre-release v0.1.0, group-threshold-cryptography-pre-release v0.1.0, ferveo-pre-release v0.2.0 ([`ffb9b21`](https://github.com/gear-tech/ferveo-nucypher/commit/ffb9b21619d0f5dc0fb309bf2f493d3c0c25e1f0))
    - Release ferveo-common-pre-release v0.1.0, subproductdomain-pre-release v0.1.0, group-threshold-cryptography-pre-release v0.1.0, ferveo-pre-release v0.2.0 ([`a7b889e`](https://github.com/gear-tech/ferveo-nucypher/commit/a7b889e3a20cfffc96bcb801dfb0946227cb32d9))
    - Adjust changelogs for cargo-smart-release ([`ca43921`](https://github.com/gear-tech/ferveo-nucypher/commit/ca43921af214903e2d1345bb05b5f9c6e1987919))
    - Release 0.1.0 crate versions ([`c02e305`](https://github.com/gear-tech/ferveo-nucypher/commit/c02e3050b7a9dcf0260a5eb4e42ff74f3788c3bf))
    - Merge pull request #134 from piotr-roslaniec/remove-ftt-opt ([`2338213`](https://github.com/gear-tech/ferveo-nucypher/commit/23382139265bc043769d41f4da9e0998f9ba9757))
    - Use general evaluation domain ([`2c20efb`](https://github.com/gear-tech/ferveo-nucypher/commit/2c20efb59d7d1075d6b1413b2ae7fbb55c422143))
    - Fix using bad number of domain points ([`d5ec5e0`](https://github.com/gear-tech/ferveo-nucypher/commit/d5ec5e0f9d1303e51a805c4dafbab7ed2efcb7be))
    - Merge pull request #119 from nucypher/nucypher-core-integration ([`52c1f27`](https://github.com/gear-tech/ferveo-nucypher/commit/52c1f27627798fa266d2e5079f5121cc71e8e284))
    - Merge pull request #118 from nucypher/expose-bindings-from-main-crate ([`11d6cea`](https://github.com/gear-tech/ferveo-nucypher/commit/11d6ceaf26f45c76dec0c5a9fcf5eae5301502d3))
    - Release pre-release crates ([`8df87ff`](https://github.com/gear-tech/ferveo-nucypher/commit/8df87ff36ac81bd9e60013cda892d31ddf402868))
    - Update crates to 2021 edition #111 ([`591c05e`](https://github.com/gear-tech/ferveo-nucypher/commit/591c05e64ef9d2f7218418b6aa9d33181c60c88f))
    - Merge pull request #102 from piotr-roslaniec/local-verification-wasm ([`aacdf04`](https://github.com/gear-tech/ferveo-nucypher/commit/aacdf0462d73720e97c1d7924fc49e3d252a691a))
    - Js bindings fail to correctly decrypt the ciphertext ([`ae79060`](https://github.com/gear-tech/ferveo-nucypher/commit/ae790601f691a7727489dbd8606dcd6ed0e4106d))
    - Js bindings fail to correctly decrypt the ciphertext ([`3e7db72`](https://github.com/gear-tech/ferveo-nucypher/commit/3e7db72e5878bfc54b0324c4c79a2a058fc9e0e9))
    - Merge pull request #75 from nucypher/release-ferveo-py ([`2529f74`](https://github.com/gear-tech/ferveo-nucypher/commit/2529f743fe6f07935938cbef81faa0230e478f87))
    - Merge pull request #56 from nucypher/ferveo-light-tdec ([`8fa25b6`](https://github.com/gear-tech/ferveo-nucypher/commit/8fa25b66bf32585b2ef406bbec3999fd9ce75225))
    - Merge pull request #62 from nucypher/client-server-api ([`3a6e3c4`](https://github.com/gear-tech/ferveo-nucypher/commit/3a6e3c4b59c192289f86c0e37f119b29ccd3d620))
    - Merge pull request #67 from nucypher/arkworks-0.4 ([`bd78f97`](https://github.com/gear-tech/ferveo-nucypher/commit/bd78f9741246a2118bf6e3fdf48c72d6adf51b9e))
    - Merge pull request #68 from nucypher/error-handling ([`093f17e`](https://github.com/gear-tech/ferveo-nucypher/commit/093f17e22f606b33a468bd62ad37cf22f3dda265))
    - Merge branch 'error-handling' into tpke-wasm-api-example ([`707f460`](https://github.com/gear-tech/ferveo-nucypher/commit/707f460666acc2781d6dcfa49e0f75f1159f466f))
    - Replace cargo-udeps with cargo-machete ([`9d38a03`](https://github.com/gear-tech/ferveo-nucypher/commit/9d38a03f0f229ff91c5c9d21cc290b30e88ad993))
    - Merge branch 'error-handling' into release-ferveo-py ([`d2a0ca0`](https://github.com/gear-tech/ferveo-nucypher/commit/d2a0ca045beb4dd298f2c06b20b313456a1e81f9))
    - Sketch error handling in ferveo ([`a68d2d9`](https://github.com/gear-tech/ferveo-nucypher/commit/a68d2d9b62414fd06afa234f240508d1c41e68a8))
    - Self review ([`2d926de`](https://github.com/gear-tech/ferveo-nucypher/commit/2d926de9a96a9492063fe4ad69a4dee51d5cae88))
    - Update arkworks to 0.4.0 - first pass ([`b1999b8`](https://github.com/gear-tech/ferveo-nucypher/commit/b1999b86a2b04c719ec29b1263612de88a0cfd49))
    - Fix import style ([`6d92b01`](https://github.com/gear-tech/ferveo-nucypher/commit/6d92b010139b915da1a89ffa686bf24871c7afd1))
    - Merge branch 'main' into use-sha256 ([`fa1c1a8`](https://github.com/gear-tech/ferveo-nucypher/commit/fa1c1a8bf2b338cb379a481d8b042c45af23c470))
    - Merge pull request #27 from nucypher/dkg-pvss-flow ([`e842b8a`](https://github.com/gear-tech/ferveo-nucypher/commit/e842b8a5bb2cafe2e768ca29e5f0210f969ea748))
    - Fix clippy ([`cca3270`](https://github.com/gear-tech/ferveo-nucypher/commit/cca32700b3b13aafab6fcb899f852d3643dddcfd))
    - Fix clippy ([`7cad9ae`](https://github.com/gear-tech/ferveo-nucypher/commit/7cad9aea331ed8e510bca6afd043fe61a466ef08))
</details>

## v0.4.0 (2025-08-15)

<csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/>
<csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/>

### Chore

 - <csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/> move shared dependencies to workspace crate
 - <csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/> adjust changelogs for cargo-smart-release

## v0.1.0 (2023-07-07)

<csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/>

### Chore

 - <csr-id-ca43921af214903e2d1345bb05b5f9c6e1987919/> adjust changelogs for cargo-smart-release

