# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (2026-06-11)

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

### New Features

 - <csr-id-cb0750a69560a21ceab8c7a55ead3d1ff8079161/> implement dealer

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

 - 75 commits contributed to the release.
 - 18 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#7](https://github.com/gear-tech/ferveo-nucypher/issues/7)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#7](https://github.com/gear-tech/ferveo-nucypher/issues/7)**
    - Prepare crates to publish ([`379dabe`](https://github.com/gear-tech/ferveo-nucypher/commit/379dabea4ab9c9e428a69c6683eafe2447ebf788))
 * **Uncategorized**
    - Merge pull request #6 from gear-tech/feat/rename-ferveo-tdec ([`d27693b`](https://github.com/gear-tech/ferveo-nucypher/commit/d27693badb55c29c1e337d628d0ef886ec7b5319))
    - Rename ferveo-nucypher-tdec -> ferveo-gear-tdec ([`186c053`](https://github.com/gear-tech/ferveo-nucypher/commit/186c053ab725b2eea3e85ee96657e8955b16deaf))
    - Merge pull request #4 from gear-tech/minor-fixed-tdec ([`caa532a`](https://github.com/gear-tech/ferveo-nucypher/commit/caa532a5959bd55adc22796191cb8c68f6686f4a))
    - Simple test for serialize-deserialize for ciphertext ([`df5fa3b`](https://github.com/gear-tech/ferveo-nucypher/commit/df5fa3b117d20043344614c45ccf6484428c8203))
    - Fix serialize/deserialize with serde-hex ([`ea186a8`](https://github.com/gear-tech/ferveo-nucypher/commit/ea186a8b45fdf3e647f973b5523626af1ae001e8))
    - Reexport test-rng from ark-std ([`4bfc8e5`](https://github.com/gear-tech/ferveo-nucypher/commit/4bfc8e559856c21ccfd729ce58caad3c84ff5247))
    - Add bls12_381 SharedSecret ([`8438972`](https://github.com/gear-tech/ferveo-nucypher/commit/8438972f412b01258142c646c1b7d3c84d3e46ee))
    - Merge pull request #2 from gear-tech/chore/better-use-in-ethexe ([`c4be797`](https://github.com/gear-tech/ferveo-nucypher/commit/c4be7971ea4705e72f03c72777de4fea0e720a77))
    - Fix +nightly fmt ([`50a9861`](https://github.com/gear-tech/ferveo-nucypher/commit/50a9861635787aef175d1fa96c91773920ae62f1))
    - Remove useless SetupParams ([`f1535e1`](https://github.com/gear-tech/ferveo-nucypher/commit/f1535e1fd8ec994080c45dcfe947df257f5fe532))
    - Make logic prettier ([`c70d260`](https://github.com/gear-tech/ferveo-nucypher/commit/c70d2605882a9314abcc075fa6df117892ea532e))
    - Serialize/deserialize for G1, G2, GT points ([`1f00d41`](https://github.com/gear-tech/ferveo-nucypher/commit/1f00d41e5aa7aa536b956fc5863a0462a9b7e188))
    - Implement dealer ([`cb0750a`](https://github.com/gear-tech/ferveo-nucypher/commit/cb0750a69560a21ceab8c7a55ead3d1ff8079161))
    - Serialize G1Affine and G2Affine points ([`007ba3f`](https://github.com/gear-tech/ferveo-nucypher/commit/007ba3fbebf99e3b9f45b7054d471158dceaadf7))
    - Dirty implementation Encode/Decode for Ciphertext ([`18d49bc`](https://github.com/gear-tech/ferveo-nucypher/commit/18d49bcb39ede98b80873e2b26b2e3b4ba0401e0))
    - Add codec skip for phantom field ([`edeadce`](https://github.com/gear-tech/ferveo-nucypher/commit/edeadce84c41a75646c3a3a0f53efb87db65c3f5))
    - Add parity-scale-codec for Ciphertext ([`b88f4bc`](https://github.com/gear-tech/ferveo-nucypher/commit/b88f4bc90487dada611b2954005ca931c11d4fcf))
    - Add derive Hash ([`187ca2f`](https://github.com/gear-tech/ferveo-nucypher/commit/187ca2fc581d74028ac2924d45af750479318937))
    - Rand -> rand_traits ([`cad43a8`](https://github.com/gear-tech/ferveo-nucypher/commit/cad43a88c3c247ac2ca1fa583be913ba3febcf6b))
    - Add re-export for rand ([`79b3e3e`](https://github.com/gear-tech/ferveo-nucypher/commit/79b3e3e399e1724ddb231e82e89e199563801667))
    - Add dkg public key to bls12_381 public exports ([`3f7e005`](https://github.com/gear-tech/ferveo-nucypher/commit/3f7e0058aa6d1de4ed2b418d0b024ac54401acd4))
    - Merge pull request #1 from gear-tech/chore/ark-0.6-and-cleanup ([`4f7f8f9`](https://github.com/gear-tech/ferveo-nucypher/commit/4f7f8f9855aafc19bb93c6dd367b6c057d5b65d0))
    - Patch ferveo-tdec for typed encrypt/decrypt value ([`43d6059`](https://github.com/gear-tech/ferveo-nucypher/commit/43d6059d07ac045ca15a0275ae1a490fa43fdc0c))
    - Add encrypt/decrypt for encodable value ([`20b6dce`](https://github.com/gear-tech/ferveo-nucypher/commit/20b6dce622b347d92a7e39b7f2d1c08a688da6a5))
    - Better DX | remove ugly SecretBox ([`ec4bd3f`](https://github.com/gear-tech/ferveo-nucypher/commit/ec4bd3f5ee587b4c6473bbdd0442e81f45c5801f))
    - Fix public api ([`8c15fce`](https://github.com/gear-tech/ferveo-nucypher/commit/8c15fce4fd9b9ad5db6689be56ccaee8db967ed5))
    - Bump arkworks to 0.6, edition 2024, remove python/wasm bindings ([`be89332`](https://github.com/gear-tech/ferveo-nucypher/commit/be89332821d2fa301519787075e5454963271087))
    - Release ferveo-nucypher-common v0.4.0, subproductdomain-nucypher v0.4.0, ferveo-nucypher-tdec v0.4.0, ferveo-nucypher v0.4.0 ([`b50d448`](https://github.com/gear-tech/ferveo-nucypher/commit/b50d448521b5fb29c630af4c9a3994f3b40060c8))
    - Merge pull request #188 from nucypher/rocknroll ([`1e66268`](https://github.com/gear-tech/ferveo-nucypher/commit/1e66268dfbfbf76566b4bcf6c25a9852692bb380))
    - Merge pull request #211 from derekpierre/mrkrabs ([`763e06b`](https://github.com/gear-tech/ferveo-nucypher/commit/763e06bb2375e2ded95b409e282ae1f491e16d59))
    - Merge pull request #205 from cygnusv/mrkrabs ([`bb51e96`](https://github.com/gear-tech/ferveo-nucypher/commit/bb51e963f552d2ced387d0ac5c4b311f13715eb4))
    - Update cargo.toml of all ferveo packages for public release. ([`d21ea18`](https://github.com/gear-tech/ferveo-nucypher/commit/d21ea1826f81f47ee88a64dcb98678560e691e57))
    - Fix incorrect selected participants size that caused benchmarking to fail. ([`24fac18`](https://github.com/gear-tech/ferveo-nucypher/commit/24fac1860435045b736c8078c8f3fbc5806fd9bc))
    - Update cargo.toml of all ferveo packages for test release. ([`000dc17`](https://github.com/gear-tech/ferveo-nucypher/commit/000dc1715c31f2a32f2366feb6ca652b57d40130))
    - Sq update wasm ([`c4eaa4a`](https://github.com/gear-tech/ferveo-nucypher/commit/c4eaa4a76f3d93075cefea9d1d19066466ba3b6d))
    - Update wasm-bindgen ([`19e228b`](https://github.com/gear-tech/ferveo-nucypher/commit/19e228b70920b359d93175dfcc5470062832102c))
    - Update cargo.toml of all ferveo packages ([`4e03d43`](https://github.com/gear-tech/ferveo-nucypher/commit/4e03d43255c2fceb729bf2227bff396a25d700c5))
    - Update authors ([`380e984`](https://github.com/gear-tech/ferveo-nucypher/commit/380e9840f0b491da002ff02b863230f5824b500e))
    - Refactor domain points ([`70ac464`](https://github.com/gear-tech/ferveo-nucypher/commit/70ac4642ad2545114a4ff2a982a11ce764112fd0))
    - Merge pull request #186 from cygnusv/spongebob ([`bc64858`](https://github.com/gear-tech/ferveo-nucypher/commit/bc6485811b40b1025115159a2504f49fac4789a8))
    - Link some TODOs and FIXMEs with issues ([`f7a0065`](https://github.com/gear-tech/ferveo-nucypher/commit/f7a00658cd121c2c1304d3ea628240765053515d))
    - Remove generator inverse from API ([`bf1cf0f`](https://github.com/gear-tech/ferveo-nucypher/commit/bf1cf0fd965edb3e7530ccefab428d1dad08c9dd))
    - Remove unnecessary code in context.rs ([`0efb567`](https://github.com/gear-tech/ferveo-nucypher/commit/0efb567655f681d6f007fe1624c7d60515d0423b))
    - Code areas marked for refactor or removal ([`35eb653`](https://github.com/gear-tech/ferveo-nucypher/commit/35eb65318e24e689bb5370895b75aa7ab2827eaa))
    - Consider encrypt_in_place for AEAD ([`ee98c24`](https://github.com/gear-tech/ferveo-nucypher/commit/ee98c249c0bba582af26d304d329e69676e97d45))
    - Consider using multipairings ([`a3f607d`](https://github.com/gear-tech/ferveo-nucypher/commit/a3f607dcf5961973ad365f5bb5ed14d5272d3547))
    - Use PublicKeys instead of internal G2 type when possible ([`8296118`](https://github.com/gear-tech/ferveo-nucypher/commit/8296118807587b04a6773c9edb2116635c1a349a))
    - Explicitly rename DKG PublicKeys to avoid confusion with Validator PKs ([`dceac71`](https://github.com/gear-tech/ferveo-nucypher/commit/dceac71f876f4f5f487aa3538697efa35a64d861))
    - Add TODO about using explicit imports (see #194) ([`cff8dfd`](https://github.com/gear-tech/ferveo-nucypher/commit/cff8dfd2940a70d595d959b417f7cec16c57a4eb))
    - Assorted cleanup ([`b3df880`](https://github.com/gear-tech/ferveo-nucypher/commit/b3df8808f391cb1710be507725277e3ad08a6bdc))
    - PrivateKeys are never blinded directly ([`b8a4c5c`](https://github.com/gear-tech/ferveo-nucypher/commit/b8a4c5ca0ec40bc14a541c087f8b2e85cc0c8297))
    - Tidy up imports in several places ([`8a52e07`](https://github.com/gear-tech/ferveo-nucypher/commit/8a52e07e2883794fa945be04d82af6301a48bf19))
    - Pass Keypairs as input to unblind BlindedKeyShares ([`bad0d3b`](https://github.com/gear-tech/ferveo-nucypher/commit/bad0d3bf1aad626c4b6af7cf0ffa8f83654728f1))
    - Some tests fixed: share updating should be done on top of blinded shares ([`ec9e368`](https://github.com/gear-tech/ferveo-nucypher/commit/ec9e3687799526c2567321cfa981e823e150204a))
    - Yay! Tests work when blinding is deactivated, so the problem is unblinding ([`ba6cd93`](https://github.com/gear-tech/ferveo-nucypher/commit/ba6cd93670403ac0ea4a64e87cb49c535b46dcaa))
    - Clarifying some refresh tests ([`1020d00`](https://github.com/gear-tech/ferveo-nucypher/commit/1020d007afd8472bde2da93d16a9a5d58df80b24))
    - Distinction between ShareCommitments and TDec PublicKeys ([`0cfa02e`](https://github.com/gear-tech/ferveo-nucypher/commit/0cfa02e836796a894ea0cecec70bce34ffae30e4))
    - Merge pull request #189 from piotr-roslaniec/workspace-deps ([`be98542`](https://github.com/gear-tech/ferveo-nucypher/commit/be9854252fdff297d99a63eb443a473ecfd41f5a))
    - Move shared dependencies to workspace crate ([`983110c`](https://github.com/gear-tech/ferveo-nucypher/commit/983110c4dbb41eb7f0fba2c06f561b68718d0f29))
    - Merge pull request #187 from piotr-roslaniec/remove-fast-variant ([`b72a338`](https://github.com/gear-tech/ferveo-nucypher/commit/b72a33803852bfaf444d6c2c4a278f93f334ab89))
    - Remove fast variant ([`6e3369d`](https://github.com/gear-tech/ferveo-nucypher/commit/6e3369d11cfd4ec751775e1eee82f8192b51943e))
    - Merge pull request #185 from piotr-roslaniec/aggregate-from-subset ([`299a471`](https://github.com/gear-tech/ferveo-nucypher/commit/299a471d2ee658ca374c3400ccac8fd24bb8d1a1))
    - Merge pull request #183 from piotr-roslaniec/remove-dkg-state ([`aa69b36`](https://github.com/gear-tech/ferveo-nucypher/commit/aa69b364a57c511f96f8c2f1b1f0c36ab2309e50))
    - Not using subset of participants in precomputed variant ([`975dae0`](https://github.com/gear-tech/ferveo-nucypher/commit/975dae0d5f8d1a2e5c061fbc8d11b1cc73c867d7))
    - Fix tests sensitive to message ordering ([`4a8375d`](https://github.com/gear-tech/ferveo-nucypher/commit/4a8375d1873560241ae8eea96230a42635ed1764))
    - Merge pull request #175 from piotr-roslaniec/rewrite-refreshing ([`2c97934`](https://github.com/gear-tech/ferveo-nucypher/commit/2c97934251c04754b8c5353492823e3a97dc53a9))
    - Rename public key share to public key ([`0ef7de4`](https://github.com/gear-tech/ferveo-nucypher/commit/0ef7de4c9b4442e2c6125d457de9420146be50b7))
    - Remove state from dkg, part 1 ([`315d2b4`](https://github.com/gear-tech/ferveo-nucypher/commit/315d2b4cc2825e13820d9c64639490c44b538385))
    - Introduce refreshing api in ferveo ([`4713848`](https://github.com/gear-tech/ferveo-nucypher/commit/47138489bc9567674b57d61b0d105ff6c1c7cb6c))
    - Avoid using crypto primitives directly, part 1 ([`8b26396`](https://github.com/gear-tech/ferveo-nucypher/commit/8b26396cc26ceeddca52dc37ac9461f0bb93ecfe))
    - Merge pull request #171 from piotr-roslaniec/python-versions ([`de9cf36`](https://github.com/gear-tech/ferveo-nucypher/commit/de9cf36ad88a0242e43bbc6339eb840b6d97d88c))
    - Remove duplicated field ([`802e712`](https://github.com/gear-tech/ferveo-nucypher/commit/802e7121d5eb5a31617bf88c4e14fe79d45e68e3))
    - Merge pull request #166 from nucypher/chores ([`7350d91`](https://github.com/gear-tech/ferveo-nucypher/commit/7350d91708af55b5aa939a3f7e9cd62e7de7359a))
    - Rename ferveo-tpke package to ferveo-tdec ([`58002f5`](https://github.com/gear-tech/ferveo-nucypher/commit/58002f50155df31a11b9d58d94750a2ed1076102))
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

