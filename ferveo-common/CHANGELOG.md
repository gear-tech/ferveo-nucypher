# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (2026-06-29)

<csr-id-379dabea4ab9c9e428a69c6683eafe2447ebf788/>
<csr-id-be89332821d2fa301519787075e5454963271087/>
<csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/>
<csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/>
<csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/>
<csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/>
<csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/>
<csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/>

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
 - <csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/> adjust changelogs for cargo-smart-release

### Chore

 - <csr-id-979a12c0a4600283ffdf9aafcdb97d2126814596/> serde Serialize/Deserialize for generic types over E: Pairing
   * update serialize/deserialize paths
   
   * serde bound for types generic over E: Pairing

### New Features

 - <csr-id-52efe010264bdd5978111e148190359b9383d53e/> derive eq in DkgPublicKey
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

### Other

 - <csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/> introduce refreshing api in ferveo
 - <csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/> Made ferveo-common wasm compatible (a tiny change). Fixes a world of pain upstream in Anoma
 - <csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/> Formatting
 - <csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/> Removed the announce phase from the dkg

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
<csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/>
<csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/>
<csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/>
<csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/>
<csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/>

### Chore

 - <csr-id-983110c4dbb41eb7f0fba2c06f561b68718d0f29/> move shared dependencies to workspace crate
 - <csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/> adjust changelogs for cargo-smart-release

### New Features

 - <csr-id-52efe010264bdd5978111e148190359b9383d53e/> derive eq in DkgPublicKey

### Other

 - <csr-id-47138489bc9567674b57d61b0d105ff6c1c7cb6c/> introduce refreshing api in ferveo
 - <csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/> Made ferveo-common wasm compatible (a tiny change). Fixes a world of pain upstream in Anoma
 - <csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/> Formatting
 - <csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/> Removed the announce phase from the dkg

## 0.1.1 (2023-08-28)

Maintenance release

## v0.1.0 (2023-07-07)

<csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/>
<csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/>
<csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/>
<csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/>

### Other

 - <csr-id-ab6701666e3b05bd783ce0309025e842fa83e4c1/> Made ferveo-common wasm compatible (a tiny change). Fixes a world of pain upstream in Anoma
 - <csr-id-d786fae33b01cd0863f29b70810dfcc847f2542b/> Formatting
 - <csr-id-ec58fe1828d0560525c80cd1dc4013915b0ac54e/> Removed the announce phase from the dkg

### Chore

 - <csr-id-0eb5bd48b598709dd0fc54adb424f5f41ce52e92/> adjust changelogs for cargo-smart-release

