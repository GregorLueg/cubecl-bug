# This repo was generated to showcase a very funky bug

Seems to be solved, see [here](https://github.com/tracel-ai/cubecl/issues/1375) 

## Problem statement

For reproduction and easier solving of a bug in cubecl `"0.10.0"` in interaction
with the Apple Metal backend.

```bash
cargo run --release
    Finished `release` profile [optimized] target(s) in 0.20s
     Running `target/release/cubecl-bug`
out[0] = 0  // a != b via expr-if   (expect 1) <- offending test
out[1] = 0  // a == b via expr-if   (expect 0)
out[2] = 1  // a != b via stmt-if   (expect 1)
out[3] = 1  // 1 != 0 via expr-if   (expect 1)
out[4] = 494  // while i < b counter  (expect 494)
out[5] = 130  // a                     (expect 130)
out[6] = 494  // b                     (expect 494)
```

In one sentence: on the wgpu/Metal backend, an expression-position if/else with
a runtime condition evaluates to its else branch regardless of the condition,
while statement-if, comptime-condition expression-if, and while conditions over
the same comparison all behave correctly.

## System specs

**MacOS version used**: 26.5.1 (25F80)
**Hardware used**: MacBook Pro Apple M1 Max

## Additional info

```bash
cargo tree -p cubecl
cubecl v0.10.0
├── cubecl-core v0.10.0
│   ├── bitflags v2.13.0
│   │   └── serde_core v1.0.228
│   ├── bytemuck v1.25.0
│   │   └── bytemuck_derive v1.10.2 (proc-macro)
│   │       ├── proc-macro2 v1.0.106
│   │       │   └── unicode-ident v1.0.24
│   │       ├── quote v1.0.45
│   │       │   └── proc-macro2 v1.0.106 (*)
│   │       └── syn v2.0.117
│   │           ├── proc-macro2 v1.0.106 (*)
│   │           ├── quote v1.0.45 (*)
│   │           └── unicode-ident v1.0.24
│   ├── cubecl-common v0.10.0
│   │   ├── backtrace v0.3.76
│   │   │   ├── addr2line v0.25.1
│   │   │   │   └── gimli v0.32.3
│   │   │   ├── cfg-if v1.0.4
│   │   │   ├── libc v0.2.186
│   │   │   ├── miniz_oxide v0.8.9
│   │   │   │   └── adler2 v2.0.1
│   │   │   ├── object v0.37.3
│   │   │   │   └── memchr v2.8.1
│   │   │   └── rustc-demangle v0.1.27
│   │   ├── bytemuck v1.25.0 (*)
│   │   ├── cfg-if v1.0.4
│   │   ├── derive-new v0.7.0 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   ├── quote v1.0.45 (*)
│   │   │   └── syn v2.0.117 (*)
│   │   ├── derive_more v2.1.1
│   │   │   └── derive_more-impl v2.1.1 (proc-macro)
│   │   │       ├── convert_case v0.10.0
│   │   │       │   └── unicode-segmentation v1.13.3
│   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │       ├── quote v1.0.45 (*)
│   │   │       ├── syn v2.0.117 (*)
│   │   │       └── unicode-xid v0.2.6
│   │   │       [build-dependencies]
│   │   │       └── rustc_version v0.4.1
│   │   │           └── semver v1.0.28
│   │   ├── dirs v6.0.0
│   │   │   └── dirs-sys v0.5.0
│   │   │       ├── libc v0.2.186
│   │   │       └── option-ext v0.2.0
│   │   ├── embassy-futures v0.1.2
│   │   ├── float4 v0.2.0
│   │   ├── float8 v0.7.0
│   │   │   └── half v2.7.1
│   │   │       ├── bytemuck v1.25.0 (*)
│   │   │       ├── cfg-if v1.0.4
│   │   │       ├── num-traits v0.2.19
│   │   │       │   └── libm v0.2.16
│   │   │       │   [build-dependencies]
│   │   │       │   └── autocfg v1.5.1
│   │   │       ├── serde v1.0.228
│   │   │       │   ├── serde_core v1.0.228
│   │   │       │   └── serde_derive v1.0.228 (proc-macro)
│   │   │       │       ├── proc-macro2 v1.0.106 (*)
│   │   │       │       ├── quote v1.0.45 (*)
│   │   │       │       └── syn v2.0.117 (*)
│   │   │       └── zerocopy v0.8.50
│   │   │           └── zerocopy-derive v0.8.50 (proc-macro)
│   │   │               ├── proc-macro2 v1.0.106 (*)
│   │   │               ├── quote v1.0.45 (*)
│   │   │               └── syn v2.0.117 (*)
│   │   ├── futures-lite v2.6.1
│   │   │   ├── fastrand v2.4.1
│   │   │   ├── futures-core v0.3.32
│   │   │   ├── futures-io v0.3.32
│   │   │   ├── parking v2.2.1
│   │   │   └── pin-project-lite v0.2.17
│   │   ├── half v2.7.1 (*)
│   │   ├── hashbrown v0.16.1
│   │   │   ├── allocator-api2 v0.2.21
│   │   │   ├── equivalent v1.0.2
│   │   │   ├── foldhash v0.2.0
│   │   │   └── serde_core v1.0.228
│   │   ├── log v0.4.32
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── oneshot v0.2.1
│   │   ├── parking_lot v0.12.5
│   │   │   ├── lock_api v0.4.14
│   │   │   │   └── scopeguard v1.2.0
│   │   │   └── parking_lot_core v0.9.12
│   │   │       ├── cfg-if v1.0.4
│   │   │       ├── libc v0.2.186
│   │   │       └── smallvec v1.15.1
│   │   │           └── serde v1.0.228 (*)
│   │   ├── rand v0.10.1
│   │   │   ├── chacha20 v0.10.0
│   │   │   │   ├── cfg-if v1.0.4
│   │   │   │   └── rand_core v0.10.1
│   │   │   ├── getrandom v0.4.2
│   │   │   │   ├── cfg-if v1.0.4
│   │   │   │   ├── libc v0.2.186
│   │   │   │   └── rand_core v0.10.1
│   │   │   └── rand_core v0.10.1
│   │   ├── sanitize-filename v0.6.0
│   │   │   └── regex v1.12.3
│   │   │       ├── regex-automata v0.4.14
│   │   │       │   └── regex-syntax v0.8.10
│   │   │       └── regex-syntax v0.8.10
│   │   ├── serde v1.0.228 (*)
│   │   ├── serde_bytes v0.11.19
│   │   │   └── serde_core v1.0.228
│   │   ├── serde_json v1.0.150
│   │   │   ├── itoa v1.0.18
│   │   │   ├── memchr v2.8.1
│   │   │   ├── serde_core v1.0.228
│   │   │   └── zmij v1.0.21
│   │   ├── spin v0.10.0
│   │   │   └── lock_api v0.4.14 (*)
│   │   ├── toml v1.1.2+spec-1.1.0
│   │   │   ├── serde_core v1.0.228
│   │   │   ├── serde_spanned v1.1.1
│   │   │   │   └── serde_core v1.0.228
│   │   │   ├── toml_datetime v1.1.1+spec-1.1.0
│   │   │   │   └── serde_core v1.0.228
│   │   │   ├── toml_parser v1.1.2+spec-1.1.0
│   │   │   │   └── winnow v1.0.3
│   │   │   ├── toml_writer v1.1.1+spec-1.1.0
│   │   │   └── winnow v1.0.3
│   │   ├── tynm v0.2.0
│   │   │   └── nom v8.0.0
│   │   │       └── memchr v2.8.1
│   │   ├── web-time v1.1.0
│   │   └── xxhash-rust v0.8.15
│   │   [build-dependencies]
│   │   └── cfg_aliases v0.2.1
│   ├── cubecl-ir v0.10.0
│   │   ├── cubecl-common v0.10.0 (*)
│   │   ├── cubecl-macros-internal v0.10.0 (proc-macro)
│   │   │   ├── darling v0.23.0
│   │   │   │   ├── darling_core v0.23.0
│   │   │   │   │   ├── ident_case v1.0.1
│   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   ├── strsim v0.11.1
│   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   └── darling_macro v0.23.0 (proc-macro)
│   │   │   │       ├── darling_core v0.23.0 (*)
│   │   │   │       ├── quote v1.0.45 (*)
│   │   │   │       └── syn v2.0.117 (*)
│   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   ├── quote v1.0.45 (*)
│   │   │   └── syn v2.0.117 (*)
│   │   ├── derive-new v0.7.0 (proc-macro) (*)
│   │   ├── derive_more v2.1.1 (*)
│   │   ├── enumset v1.1.13
│   │   │   ├── enumset_derive v0.15.0 (proc-macro)
│   │   │   │   ├── darling v0.21.3
│   │   │   │   │   ├── darling_core v0.21.3
│   │   │   │   │   │   ├── fnv v1.0.7
│   │   │   │   │   │   ├── ident_case v1.0.1
│   │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   │   │   └── syn v2.0.117 (*)
│   │   │   │   │   └── darling_macro v0.21.3 (proc-macro)
│   │   │   │   │       ├── darling_core v0.21.3 (*)
│   │   │   │   │       ├── quote v1.0.45 (*)
│   │   │   │   │       └── syn v2.0.117 (*)
│   │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   │   ├── quote v1.0.45 (*)
│   │   │   │   └── syn v2.0.117 (*)
│   │   │   └── serde v1.0.228 (*)
│   │   ├── float-ord v0.3.2
│   │   ├── fnv v1.0.7
│   │   ├── foldhash v0.2.0
│   │   ├── half v2.7.1 (*)
│   │   ├── hashbrown v0.16.1 (*)
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── portable-atomic v1.13.1
│   │   │   └── serde v1.0.228 (*)
│   │   ├── serde v1.0.228 (*)
│   │   └── variadics_please v1.1.0 (proc-macro)
│   │       ├── proc-macro2 v1.0.106 (*)
│   │       ├── quote v1.0.45 (*)
│   │       └── syn v2.0.117 (*)
│   ├── cubecl-macros v0.10.0 (proc-macro)
│   │   ├── cubecl-common v0.10.0
│   │   │   ├── bytemuck v1.25.0
│   │   │   │   └── bytemuck_derive v1.10.2 (proc-macro) (*)
│   │   │   ├── cfg-if v1.0.4
│   │   │   ├── derive-new v0.7.0 (proc-macro) (*)
│   │   │   ├── derive_more v2.1.1
│   │   │   │   └── derive_more-impl v2.1.1 (proc-macro) (*)
│   │   │   ├── embassy-futures v0.1.2
│   │   │   ├── half v2.7.1
│   │   │   │   ├── cfg-if v1.0.4
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── serde v1.0.228
│   │   │   │   │   ├── serde_core v1.0.228
│   │   │   │   │   └── serde_derive v1.0.228 (proc-macro) (*)
│   │   │   │   └── zerocopy v0.8.50 (*)
│   │   │   ├── hashbrown v0.16.1
│   │   │   │   ├── allocator-api2 v0.2.21
│   │   │   │   ├── equivalent v1.0.2
│   │   │   │   └── foldhash v0.2.0
│   │   │   ├── log v0.4.32
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── rand v0.10.1
│   │   │   │   ├── chacha20 v0.10.0 (*)
│   │   │   │   └── rand_core v0.10.1
│   │   │   ├── serde v1.0.228 (*)
│   │   │   ├── spin v0.10.0 (*)
│   │   │   ├── tynm v0.2.0 (*)
│   │   │   └── web-time v1.1.0
│   │   │   [build-dependencies]
│   │   │   └── cfg_aliases v0.2.1
│   │   ├── darling v0.23.0 (*)
│   │   ├── derive-new v0.7.0 (proc-macro) (*)
│   │   ├── ident_case v1.0.1
│   │   ├── inflections v1.1.1
│   │   ├── prettyplease v0.2.37
│   │   │   ├── proc-macro2 v1.0.106 (*)
│   │   │   └── syn v2.0.117 (*)
│   │   ├── proc-macro2 v1.0.106 (*)
│   │   ├── quote v1.0.45 (*)
│   │   └── syn v2.0.117 (*)
│   ├── cubecl-runtime v0.10.0
│   │   ├── ahash v0.8.12
│   │   │   ├── cfg-if v1.0.4
│   │   │   ├── once_cell v1.21.4
│   │   │   └── zerocopy v0.8.50 (*)
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.5
│   │   ├── async-channel v2.5.0
│   │   │   ├── concurrent-queue v2.5.0
│   │   │   │   └── crossbeam-utils v0.8.21
│   │   │   ├── event-listener-strategy v0.5.4
│   │   │   │   ├── event-listener v5.4.1
│   │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │   │   │   │   ├── parking v2.2.1
│   │   │   │   │   └── pin-project-lite v0.2.17
│   │   │   │   └── pin-project-lite v0.2.17
│   │   │   ├── futures-core v0.3.32
│   │   │   └── pin-project-lite v0.2.17
│   │   ├── bytemuck v1.25.0 (*)
│   │   ├── cfg-if v1.0.4
│   │   ├── cubecl-common v0.10.0 (*)
│   │   ├── cubecl-ir v0.10.0 (*)
│   │   ├── cubecl-zspace v0.10.0
│   │   │   ├── derive-new v0.7.0 (proc-macro) (*)
│   │   │   ├── serde v1.0.228 (*)
│   │   │   └── smallvec v1.15.1 (*)
│   │   ├── derive-new v0.7.0 (proc-macro) (*)
│   │   ├── derive_more v2.1.1 (*)
│   │   ├── dirs v6.0.0 (*)
│   │   ├── enumset v1.1.13 (*)
│   │   ├── hashbrown v0.16.1 (*)
│   │   ├── log v0.4.32
│   │   ├── md5 v0.8.0
│   │   ├── serde v1.0.228 (*)
│   │   ├── serde_json v1.0.150 (*)
│   │   ├── spin v0.10.0 (*)
│   │   ├── thiserror v2.0.18
│   │   │   └── thiserror-impl v2.0.18 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │       ├── quote v1.0.45 (*)
│   │   │       └── syn v2.0.117 (*)
│   │   ├── toml v1.1.2+spec-1.1.0 (*)
│   │   └── web-time v1.1.0
│   │   [build-dependencies]
│   │   └── cfg_aliases v0.2.1
│   ├── cubecl-zspace v0.10.0 (*)
│   ├── derive-new v0.7.0 (proc-macro) (*)
│   ├── derive_more v2.1.1 (*)
│   ├── enumset v1.1.13 (*)
│   ├── float-ord v0.3.2
│   ├── half v2.7.1 (*)
│   ├── hashbrown v0.16.1 (*)
│   ├── log v0.4.32
│   ├── num-traits v0.2.19 (*)
│   ├── paste v1.0.15 (proc-macro)
│   ├── serde v1.0.228 (*)
│   ├── serde_json v1.0.150 (*)
│   └── variadics_please v1.1.0 (proc-macro) (*)
├── cubecl-ir v0.10.0 (*)
├── cubecl-runtime v0.10.0 (*)
├── cubecl-std v0.10.0
│   ├── cubecl-common v0.10.0 (*)
│   ├── cubecl-core v0.10.0 (*)
│   ├── cubecl-runtime v0.10.0 (*)
│   ├── half v2.7.1 (*)
│   ├── num-traits v0.2.19 (*)
│   ├── paste v1.0.15 (proc-macro)
│   ├── serde v1.0.228 (*)
│   ├── spin v0.10.0 (*)
│   └── variadics_please v1.1.0 (proc-macro) (*)
├── cubecl-wgpu v0.10.0
│   ├── async-channel v2.5.0 (*)
│   ├── bytemuck v1.25.0 (*)
│   ├── cfg-if v1.0.4
│   ├── cubecl-common v0.10.0 (*)
│   ├── cubecl-core v0.10.0 (*)
│   ├── cubecl-ir v0.10.0 (*)
│   ├── cubecl-runtime v0.10.0 (*)
│   ├── derive-new v0.7.0 (proc-macro) (*)
│   ├── derive_more v2.1.1 (*)
│   ├── half v2.7.1 (*)
│   ├── hashbrown v0.16.1 (*)
│   ├── log v0.4.32
│   ├── sanitize-filename v0.6.0 (*)
│   └── wgpu v29.0.3
│       ├── arrayvec v0.7.6
│       ├── bitflags v2.13.0 (*)
│       ├── bytemuck v1.25.0 (*)
│       ├── cfg-if v1.0.4
│       ├── document-features v0.2.12 (proc-macro)
│       │   └── litrs v1.0.0
│       ├── hashbrown v0.16.1 (*)
│       ├── log v0.4.32
│       ├── parking_lot v0.12.5 (*)
│       ├── profiling v1.0.18
│       ├── raw-window-handle v0.6.2
│       ├── smallvec v1.15.1 (*)
│       ├── static_assertions v1.1.0
│       ├── wgpu-core v29.0.3
│       │   ├── arrayvec v0.7.6
│       │   ├── bit-set v0.9.1
│       │   │   └── bit-vec v0.9.1
│       │   ├── bit-vec v0.9.1
│       │   ├── bitflags v2.13.0 (*)
│       │   ├── bytemuck v1.25.0 (*)
│       │   ├── document-features v0.2.12 (proc-macro) (*)
│       │   ├── hashbrown v0.16.1 (*)
│       │   ├── indexmap v2.14.0
│       │   │   ├── equivalent v1.0.2
│       │   │   └── hashbrown v0.17.1
│       │   ├── log v0.4.32
│       │   ├── naga v29.0.3
│       │   │   ├── arrayvec v0.7.6
│       │   │   ├── bit-set v0.9.1 (*)
│       │   │   ├── bitflags v2.13.0 (*)
│       │   │   ├── cfg-if v1.0.4
│       │   │   ├── codespan-reporting v0.13.1
│       │   │   │   └── unicode-width v0.2.2
│       │   │   ├── half v2.7.1 (*)
│       │   │   ├── hashbrown v0.16.1 (*)
│       │   │   ├── hexf-parse v0.2.1
│       │   │   ├── indexmap v2.14.0 (*)
│       │   │   ├── libm v0.2.16
│       │   │   ├── log v0.4.32
│       │   │   ├── num-traits v0.2.19 (*)
│       │   │   ├── once_cell v1.21.4
│       │   │   ├── rustc-hash v1.1.0
│       │   │   ├── spirv v0.4.0+sdk-1.4.341.0
│       │   │   │   └── bitflags v2.13.0 (*)
│       │   │   ├── thiserror v2.0.18 (*)
│       │   │   └── unicode-ident v1.0.24
│       │   │   [build-dependencies]
│       │   │   └── cfg_aliases v0.2.1
│       │   ├── once_cell v1.21.4
│       │   ├── parking_lot v0.12.5 (*)
│       │   ├── profiling v1.0.18
│       │   ├── raw-window-handle v0.6.2
│       │   ├── rustc-hash v1.1.0
│       │   ├── smallvec v1.15.1 (*)
│       │   ├── thiserror v2.0.18 (*)
│       │   ├── wgpu-core-deps-apple v29.0.3
│       │   │   └── wgpu-hal v29.0.3
│       │   │       ├── arrayvec v0.7.6
│       │   │       ├── ash v0.38.0+1.3.281
│       │   │       │   └── libloading v0.8.9
│       │   │       │       └── cfg-if v1.0.4
│       │   │       ├── bitflags v2.13.0 (*)
│       │   │       ├── block2 v0.6.2
│       │   │       │   └── objc2 v0.6.4
│       │   │       │       └── objc2-encode v4.1.0
│       │   │       ├── bytemuck v1.25.0 (*)
│       │   │       ├── cfg-if v1.0.4
│       │   │       ├── gpu-allocator v0.28.0
│       │   │       │   ├── ash v0.38.0+1.3.281 (*)
│       │   │       │   ├── hashbrown v0.16.1 (*)
│       │   │       │   ├── log v0.4.32
│       │   │       │   ├── presser v0.3.1
│       │   │       │   └── thiserror v2.0.18 (*)
│       │   │       ├── gpu-descriptor v0.3.2
│       │   │       │   ├── bitflags v2.13.0 (*)
│       │   │       │   ├── gpu-descriptor-types v0.2.0
│       │   │       │   │   └── bitflags v2.13.0 (*)
│       │   │       │   └── hashbrown v0.15.5
│       │   │       │       └── foldhash v0.1.5
│       │   │       ├── hashbrown v0.16.1 (*)
│       │   │       ├── libc v0.2.186
│       │   │       ├── libloading v0.8.9 (*)
│       │   │       ├── log v0.4.32
│       │   │       ├── naga v29.0.3 (*)
│       │   │       ├── objc2 v0.6.4 (*)
│       │   │       ├── objc2-core-foundation v0.3.2
│       │   │       │   ├── bitflags v2.13.0 (*)
│       │   │       │   └── objc2 v0.6.4 (*)
│       │   │       ├── objc2-foundation v0.3.2
│       │   │       │   ├── bitflags v2.13.0 (*)
│       │   │       │   ├── objc2 v0.6.4 (*)
│       │   │       │   └── objc2-core-foundation v0.3.2 (*)
│       │   │       ├── objc2-metal v0.3.2
│       │   │       │   ├── bitflags v2.13.0 (*)
│       │   │       │   ├── block2 v0.6.2 (*)
│       │   │       │   ├── objc2 v0.6.4 (*)
│       │   │       │   └── objc2-foundation v0.3.2 (*)
│       │   │       ├── objc2-quartz-core v0.3.2
│       │   │       │   ├── bitflags v2.13.0 (*)
│       │   │       │   ├── objc2 v0.6.4 (*)
│       │   │       │   ├── objc2-core-foundation v0.3.2 (*)
│       │   │       │   ├── objc2-foundation v0.3.2 (*)
│       │   │       │   └── objc2-metal v0.3.2 (*)
│       │   │       ├── ordered-float v5.3.0
│       │   │       │   └── num-traits v0.2.19 (*)
│       │   │       ├── parking_lot v0.12.5 (*)
│       │   │       ├── profiling v1.0.18
│       │   │       ├── raw-window-handle v0.6.2
│       │   │       ├── raw-window-metal v1.1.0
│       │   │       │   ├── objc2 v0.6.4 (*)
│       │   │       │   ├── objc2-core-foundation v0.3.2 (*)
│       │   │       │   ├── objc2-foundation v0.3.2 (*)
│       │   │       │   └── objc2-quartz-core v0.3.2 (*)
│       │   │       ├── renderdoc-sys v1.1.0
│       │   │       ├── smallvec v1.15.1 (*)
│       │   │       ├── thiserror v2.0.18 (*)
│       │   │       ├── wgpu-naga-bridge v29.0.3
│       │   │       │   ├── naga v29.0.3 (*)
│       │   │       │   └── wgpu-types v29.0.3
│       │   │       │       ├── bitflags v2.13.0 (*)
│       │   │       │       ├── bytemuck v1.25.0 (*)
│       │   │       │       ├── log v0.4.32
│       │   │       │       └── raw-window-handle v0.6.2
│       │   │       └── wgpu-types v29.0.3 (*)
│       │   │       [build-dependencies]
│       │   │       └── cfg_aliases v0.2.1
│       │   ├── wgpu-hal v29.0.3 (*)
│       │   ├── wgpu-naga-bridge v29.0.3 (*)
│       │   └── wgpu-types v29.0.3 (*)
│       │   [build-dependencies]
│       │   └── cfg_aliases v0.2.1
│       ├── wgpu-hal v29.0.3 (*)
│       └── wgpu-types v29.0.3 (*)
│       [build-dependencies]
│       └── cfg_aliases v0.2.1
│   [build-dependencies]
│   └── cfg_aliases v0.2.1
└── half v2.7.1 (*)
```

```bash
cargo tree -p wgpu
wgpu v29.0.3
├── arrayvec v0.7.6
├── bitflags v2.13.0
│   └── serde_core v1.0.228
├── bytemuck v1.25.0
│   └── bytemuck_derive v1.10.2 (proc-macro)
│       ├── proc-macro2 v1.0.106
│       │   └── unicode-ident v1.0.24
│       ├── quote v1.0.45
│       │   └── proc-macro2 v1.0.106 (*)
│       └── syn v2.0.117
│           ├── proc-macro2 v1.0.106 (*)
│           ├── quote v1.0.45 (*)
│           └── unicode-ident v1.0.24
├── cfg-if v1.0.4
├── document-features v0.2.12 (proc-macro)
│   └── litrs v1.0.0
├── hashbrown v0.16.1
│   ├── allocator-api2 v0.2.21
│   ├── equivalent v1.0.2
│   ├── foldhash v0.2.0
│   └── serde_core v1.0.228
├── log v0.4.32
├── parking_lot v0.12.5
│   ├── lock_api v0.4.14
│   │   └── scopeguard v1.2.0
│   └── parking_lot_core v0.9.12
│       ├── cfg-if v1.0.4
│       ├── libc v0.2.186
│       └── smallvec v1.15.1
│           └── serde v1.0.228
│               ├── serde_core v1.0.228
│               └── serde_derive v1.0.228 (proc-macro)
│                   ├── proc-macro2 v1.0.106 (*)
│                   ├── quote v1.0.45 (*)
│                   └── syn v2.0.117 (*)
├── profiling v1.0.18
├── raw-window-handle v0.6.2
├── smallvec v1.15.1 (*)
├── static_assertions v1.1.0
├── wgpu-core v29.0.3
│   ├── arrayvec v0.7.6
│   ├── bit-set v0.9.1
│   │   └── bit-vec v0.9.1
│   ├── bit-vec v0.9.1
│   ├── bitflags v2.13.0 (*)
│   ├── bytemuck v1.25.0 (*)
│   ├── document-features v0.2.12 (proc-macro) (*)
│   ├── hashbrown v0.16.1 (*)
│   ├── indexmap v2.14.0
│   │   ├── equivalent v1.0.2
│   │   └── hashbrown v0.17.1
│   ├── log v0.4.32
│   ├── naga v29.0.3
│   │   ├── arrayvec v0.7.6
│   │   ├── bit-set v0.9.1 (*)
│   │   ├── bitflags v2.13.0 (*)
│   │   ├── cfg-if v1.0.4
│   │   ├── codespan-reporting v0.13.1
│   │   │   └── unicode-width v0.2.2
│   │   ├── half v2.7.1
│   │   │   ├── bytemuck v1.25.0 (*)
│   │   │   ├── cfg-if v1.0.4
│   │   │   ├── num-traits v0.2.19
│   │   │   │   └── libm v0.2.16
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.5.1
│   │   │   ├── serde v1.0.228 (*)
│   │   │   └── zerocopy v0.8.50
│   │   │       └── zerocopy-derive v0.8.50 (proc-macro)
│   │   │           ├── proc-macro2 v1.0.106 (*)
│   │   │           ├── quote v1.0.45 (*)
│   │   │           └── syn v2.0.117 (*)
│   │   ├── hashbrown v0.16.1 (*)
│   │   ├── hexf-parse v0.2.1
│   │   ├── indexmap v2.14.0 (*)
│   │   ├── libm v0.2.16
│   │   ├── log v0.4.32
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── once_cell v1.21.4
│   │   ├── rustc-hash v1.1.0
│   │   ├── spirv v0.4.0+sdk-1.4.341.0
│   │   │   └── bitflags v2.13.0 (*)
│   │   ├── thiserror v2.0.18
│   │   │   └── thiserror-impl v2.0.18 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │       ├── quote v1.0.45 (*)
│   │   │       └── syn v2.0.117 (*)
│   │   └── unicode-ident v1.0.24
│   │   [build-dependencies]
│   │   └── cfg_aliases v0.2.1
│   ├── once_cell v1.21.4
│   ├── parking_lot v0.12.5 (*)
│   ├── profiling v1.0.18
│   ├── raw-window-handle v0.6.2
│   ├── rustc-hash v1.1.0
│   ├── smallvec v1.15.1 (*)
│   ├── thiserror v2.0.18 (*)
│   ├── wgpu-core-deps-apple v29.0.3
│   │   └── wgpu-hal v29.0.3
│   │       ├── arrayvec v0.7.6
│   │       ├── ash v0.38.0+1.3.281
│   │       │   └── libloading v0.8.9
│   │       │       └── cfg-if v1.0.4
│   │       ├── bitflags v2.13.0 (*)
│   │       ├── block2 v0.6.2
│   │       │   └── objc2 v0.6.4
│   │       │       └── objc2-encode v4.1.0
│   │       ├── bytemuck v1.25.0 (*)
│   │       ├── cfg-if v1.0.4
│   │       ├── gpu-allocator v0.28.0
│   │       │   ├── ash v0.38.0+1.3.281 (*)
│   │       │   ├── hashbrown v0.16.1 (*)
│   │       │   ├── log v0.4.32
│   │       │   ├── presser v0.3.1
│   │       │   └── thiserror v2.0.18 (*)
│   │       ├── gpu-descriptor v0.3.2
│   │       │   ├── bitflags v2.13.0 (*)
│   │       │   ├── gpu-descriptor-types v0.2.0
│   │       │   │   └── bitflags v2.13.0 (*)
│   │       │   └── hashbrown v0.15.5
│   │       │       └── foldhash v0.1.5
│   │       ├── hashbrown v0.16.1 (*)
│   │       ├── libc v0.2.186
│   │       ├── libloading v0.8.9 (*)
│   │       ├── log v0.4.32
│   │       ├── naga v29.0.3 (*)
│   │       ├── objc2 v0.6.4 (*)
│   │       ├── objc2-core-foundation v0.3.2
│   │       │   ├── bitflags v2.13.0 (*)
│   │       │   └── objc2 v0.6.4 (*)
│   │       ├── objc2-foundation v0.3.2
│   │       │   ├── bitflags v2.13.0 (*)
│   │       │   ├── objc2 v0.6.4 (*)
│   │       │   └── objc2-core-foundation v0.3.2 (*)
│   │       ├── objc2-metal v0.3.2
│   │       │   ├── bitflags v2.13.0 (*)
│   │       │   ├── block2 v0.6.2 (*)
│   │       │   ├── objc2 v0.6.4 (*)
│   │       │   └── objc2-foundation v0.3.2 (*)
│   │       ├── objc2-quartz-core v0.3.2
│   │       │   ├── bitflags v2.13.0 (*)
│   │       │   ├── objc2 v0.6.4 (*)
│   │       │   ├── objc2-core-foundation v0.3.2 (*)
│   │       │   ├── objc2-foundation v0.3.2 (*)
│   │       │   └── objc2-metal v0.3.2 (*)
│   │       ├── ordered-float v5.3.0
│   │       │   └── num-traits v0.2.19 (*)
│   │       ├── parking_lot v0.12.5 (*)
│   │       ├── profiling v1.0.18
│   │       ├── raw-window-handle v0.6.2
│   │       ├── raw-window-metal v1.1.0
│   │       │   ├── objc2 v0.6.4 (*)
│   │       │   ├── objc2-core-foundation v0.3.2 (*)
│   │       │   ├── objc2-foundation v0.3.2 (*)
│   │       │   └── objc2-quartz-core v0.3.2 (*)
│   │       ├── renderdoc-sys v1.1.0
│   │       ├── smallvec v1.15.1 (*)
│   │       ├── thiserror v2.0.18 (*)
│   │       ├── wgpu-naga-bridge v29.0.3
│   │       │   ├── naga v29.0.3 (*)
│   │       │   └── wgpu-types v29.0.3
│   │       │       ├── bitflags v2.13.0 (*)
│   │       │       ├── bytemuck v1.25.0 (*)
│   │       │       ├── log v0.4.32
│   │       │       └── raw-window-handle v0.6.2
│   │       └── wgpu-types v29.0.3 (*)
│   │       [build-dependencies]
│   │       └── cfg_aliases v0.2.1
│   ├── wgpu-hal v29.0.3 (*)
│   ├── wgpu-naga-bridge v29.0.3 (*)
│   └── wgpu-types v29.0.3 (*)
│   [build-dependencies]
│   └── cfg_aliases v0.2.1
├── wgpu-hal v29.0.3 (*)
└── wgpu-types v29.0.3 (*)
[build-dependencies]
└── cfg_aliases v0.2.1
```
