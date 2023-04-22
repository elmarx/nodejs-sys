# Node v19.9.0\n
affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v19.9.0/src/node_api.h)
\nupstream changelog:
* [ca981be2b9](https://github.com/nodejs/node/commit/ca981be2b9c4797af1a00c2ec74d51c4f2fac4b2) node-api: deprecate napi_module_register

# Node v19.8.0\n
affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v19.8.0/src/node_api.h)
\nupstream changelog:
* [58b1f33bd7](https://github.com/nodejs/node/commit/58b1f33bd7ac5a71b791e7f180de7d77084fb008) node-api: add __wasm32__ guards on async works

# Node v19.5.0\n
affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v19.5.0/src/js_native_api.h)
\nupstream changelog:
* [bd98e5baba](https://github.com/nodejs/node/commit/bd98e5baba78b2fb0b3eb02f6468a4f2090363da) node-api: disambiguate napi_add_finalizer

# Node v19.2.0\n
affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v19.2.0/src/node_api.h)
* [src/node_api_types.h](https://github.com/nodejs/node/blob/v19.2.0/src/node_api_types.h)
\nupstream changelog:
* [4a4f2802ec](https://github.com/nodejs/node/commit/4a4f2802ec4cb22a111363ea027ebb9be4bc8f6b) node-api: declare type napi_cleanup_hook

# Node v19.1.0\n
affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v19.1.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v19.1.0/src/js_native_api_types.h)
* [src/node_api.h](https://github.com/nodejs/node/blob/v19.1.0/src/node_api.h)
\nupstream changelog:
* [aaca54c5c0](https://github.com/nodejs/node/commit/aaca54c5c0093353afb5fbaf998014dd7e7dab3c) node-api: handle no support for external buffers
* [472edc775d](https://github.com/nodejs/node/commit/472edc775d683aed2d9ed39ca7cf381f3e7e3ce2) src: disambiguate terms used to refer to builtins and addons
* [fb744749e2](https://github.com/nodejs/node/commit/fb744749e204c349f76df79b3c513c7e0df7e4c6) node-api: explicitly set __cdecl for API functions

# Node v18.10.0
affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v18.10.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v18.10.0/src/js_native_api_types.h)
* [src/node_api.h](https://github.com/nodejs/node/blob/v18.10.0/src/node_api.h)
* [src/node_api_types.h](https://github.com/nodejs/node/blob/v18.10.0/src/node_api_types.h)
\nupstream changelog:
* [ab73cc8706](https://github.com/nodejs/node/commit/ab73cc8706b0fc5716d3b798d86a732edd5972a1) src: disambiguate terms used to refer to builtins and addons
* [037ff3da6d](https://github.com/nodejs/node/commit/037ff3da6d65954e3d3106b40a84d50851c1fbfb) node-api: explicitly set __cdecl for API functions
* [44fdf953ba](https://github.com/nodejs/node/commit/44fdf953ba435a46a3525bc877069044a3157e7d) node-api,src: fix module registration in MSVC C++
* [718be08686](https://github.com/nodejs/node/commit/718be08686312d19645861b9715491a49ee3c9cb) node-api: format Node-API related code
* [726711fe4e](https://github.com/nodejs/node/commit/726711fe4e2de7992b2ecc6921d3b083be41f950) node-api: add node_api_symbol_for()
* [4265f2769b](https://github.com/nodejs/node/commit/4265f2769bf91c0edb002da675fce98bbf61de04) src,doc: add SyntaxError napi support

# Node v17.2.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v17.2.0/src/js_native_api.h)

upstream changelog:
* [88b57bc9d3](https://github.com/nodejs/node/commit/88b57bc9d3bab7998c3de0aa787b88233cf46ed4) src,doc: add SyntaxError napi support
* [d15475578a](https://github.com/nodejs/node/commit/d15475578abfcc7e0a420719b8d55a6b3ab4f198) node-api: define version 8
* [ad3ebed046](https://github.com/nodejs/node/commit/ad3ebed046ef457530b046f2a62313a7e16b7e29) node-api: allow retrieval of add-on file name

# Node v15.12.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v15.12.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v15.12.0/src/js_native_api_types.h)
* [src/node_api.h](https://github.com/nodejs/node/blob/v15.12.0/src/node_api.h)
* [src/node_api_types.h](https://github.com/nodejs/node/blob/v15.12.0/src/node_api_types.h)

upstream changelog:
* [a86334fbb9](https://github.com/nodejs/node/commit/a86334fbb92e3776d0055563f49fad4a0f728554) node-api: define version 8

# Node v15.9.0

affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v15.9.0/src/node_api.h)

upstream changelog:
* [061939d2f6](https://github.com/nodejs/node/commit/061939d2f6fbc86ee854481dbfa0aa762a2f591f) node-api: allow retrieval of add-on file name

# Node v15.0.1

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v15.0.1/src/js_native_api.h)

upstream changelog:
* [19f14517c7](https://github.com/nodejs/node/commit/19f14517c7637b0e277b4fc3fdfe4473e5cc1262) n-api: support for object freeze/seal
* [ff38165820](https://github.com/nodejs/node/commit/ff38165820da2a9eaddabbce23f3e75aa502900b) src: allow N-API addon in `AddLinkedBinding()`
* [31b3202d59](https://github.com/nodejs/node/commit/31b3202d5902e7fa105949758677ea09643d8c5f) n-api: create N-API version 7
* [c9506a8f3e](https://github.com/nodejs/node/commit/c9506a8f3e9bc5c679151feb39198023154464ab) n-api: add more property defaults
* [0848f56cb3](https://github.com/nodejs/node/commit/0848f56cb39432090cdb99af9b8541fbc1a2849c) n-api: re-implement async env cleanup hooks
* [22cbbcf9d9](https://github.com/nodejs/node/commit/22cbbcf9d9374d4b663bf1409f292212fa57623a) n-api,src: provide asynchronous cleanup hooks
* [cc7ec889e8](https://github.com/nodejs/node/commit/cc7ec889e863433c248bc4b5c8e33f61ccc40f29) n-api: support type-tagging objects
* [b327d335ff](https://github.com/nodejs/node/commit/b327d335ff6bc48c3c2aaedccfa9c40522f6b32f) n-api: add version to wasm registration
* [b4ede54a7d](https://github.com/nodejs/node/commit/b4ede54a7d235cfe58265b2b8c455298698460a8) napi: add __wasm32__ guards
* [b18d8dde84](https://github.com/nodejs/node/commit/b18d8dde847e1bff188c6cfb2d65a96209146c2c) Revert "n-api: detect deadlocks in thread-safe function"
* [d26ca06c16](https://github.com/nodejs/node/commit/d26ca06c16f497ffa5ac4845a27922d5058a9318) n-api: detect deadlocks in thread-safe function
* [d3d5eca657](https://github.com/nodejs/node/commit/d3d5eca657474f25fab47036fef9469efc211d8a) Revert "n-api: detect deadlocks in thread-safe function"
* [aeb7084fe6](https://github.com/nodejs/node/commit/aeb7084fe6446350ec032e9819746126811bf44f) n-api: detect deadlocks in thread-safe function

# Node v14.14.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v14.14.0/src/js_native_api.h)

upstream changelog:
* [c995242068](https://github.com/nodejs/node/commit/c995242068f364292bf90c6f5a5fa6bda662896d) n-api: support for object freeze/seal

# Node v14.13.0

affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v14.13.0/src/node_api.h)

upstream changelog:
* [0d8eaa3942](https://github.com/nodejs/node/commit/0d8eaa3942f289874ed8c5d2a9468ba9c9ec45c8) src: allow N-API addon in `AddLinkedBinding()`

# v0.8.0 N-API version 7

Node.js v 14.12.0 [introduced N-API version 7](https://github.com/nodejs/node/blob/master/doc/changelogs/CHANGELOG_V14.md#14.12.0). 
So N-API version 7 is the default, but v6 (and v5) may be selected via feature-flag.

# Node v14.12.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v14.12.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v14.12.0/src/js_native_api_types.h)

upstream changelog:
* [ca1181615e](https://github.com/nodejs/node/commit/ca1181615e961ec948587aa6f8b7e46efd7cbd71) n-api: create N-API version 7
* [7f3b2b2a1f](https://github.com/nodejs/node/commit/7f3b2b2a1f2b2fa25adf9c4ea261f2a99ddd74aa) n-api: add more property defaults

# v0.7.0 `key_filter` and `property_attributes` are now bitfields

These enums are supposed to be combined using `|`, but the "rustified"
enum configuration doesn't allow it. Bindgen has a bitfield enum style
which _does_ allow `|`-ing values.

N-API currently includes two enums used as bitfields:
[`napi_key_filter`](https://nodejs.org/api/n-api.html#n_api_napi_key_filter) and 
[`napi_property_attributes`](https://nodejs.org/api/n-api.html#n_api_napi_property_attributes).

# Node v14.11.0

affected files:
* [src/node_api.h](https://github.com/nodejs/node/blob/v14.11.0/src/node_api.h)
* [src/node_api_types.h](https://github.com/nodejs/node/blob/v14.11.0/src/node_api_types.h)

upstream changelog:
* [3c32fe09e9](https://github.com/nodejs/node/commit/3c32fe09e9354479a2527bdd7484d6efab39f864) n-api: re-implement async env cleanup hooks

# v0.6.0 breaking change — `size_t` is now `usize`

`bindgen` switched the default output for parameters with the `size_t` C type to `u32`/`u64` depending on the target platform. 
Per [rust-lang/rust-bindgen#1671](https://github.com/rust-lang/rust-bindgen/issues/1671), the old behaviour where 
`size_t` would just output `usize` is not _technically_ correct, because the C standard has a slightly different definition for `size_t`.

It should be correct for all platforms that Node.js supports though. bindgen added a toggle to opt back in to the old behaviour. 
Since bindgen had been doing this for years without issue, I think we're safe. 

# Node v14.8.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v14.8.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v14.8.0/src/js_native_api_types.h)
* [src/node_api.h](https://github.com/nodejs/node/blob/v14.8.0/src/node_api.h)
* [src/node_api_types.h](https://github.com/nodejs/node/blob/v14.8.0/src/node_api_types.h)

upstream changelog:
* [8630f34776](https://github.com/nodejs/node/commit/8630f3477697835719df93dbc49d03f60cdf2b31) n-api,src: provide asynchronous cleanup hooks
* [8cc9e5eb52](https://github.com/nodejs/node/commit/8cc9e5eb52dbbff49a594c2c8c07032d0b8f6d98) n-api: support type-tagging objects

# Node v14.5.0

affected files:
* [src/js_native_api.h](https://github.com/nodejs/node/blob/v14.5.0/src/js_native_api.h)
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v14.5.0/src/js_native_api_types.h)
* [src/node_api.h](https://github.com/nodejs/node/blob/v14.5.0/src/node_api.h)

upstream changelog:
* [ac41bf03fa](https://github.com/nodejs/node/commit/ac41bf03fa6b8f1d78d8ec150481553d765ac290) n-api: add version to wasm registration
* [9148e01e76](https://github.com/nodejs/node/commit/9148e01e7612f886a6fe6563e1ad7bb20e7beac1) napi: add __wasm32__ guards
* [f4cfe94d90](https://github.com/nodejs/node/commit/f4cfe94d90c59e0e6b3cdbdad333f71c9ef20216) Revert "n-api: detect deadlocks in thread-safe function"

# Node v14.1.0

affected files:
* [src/js_native_api_types.h](https://github.com/nodejs/node/blob/v14.1.0/src/js_native_api_types.h)

upstream changelog:
* [861eb39307](https://github.com/nodejs/node/commit/861eb39307d68640305ad8cb456ecfa8ed25ffa3) n-api: detect deadlocks in thread-safe function

