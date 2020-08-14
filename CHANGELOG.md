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

