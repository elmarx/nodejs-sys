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

