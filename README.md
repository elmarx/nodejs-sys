nodejs-sys
----------

Bindings for NodeJS' [N-API](https://nodejs.org/dist/latest-v13.x/docs/api/n-api.html).

Requirements
============

To build this crate you need `libclang`, since bindings are being generated at build-time (by [bindgen](https://docs.rs/bindgen/)).


Stability
=========

This crate is probably already done/stable, the real work is to provide a wrapper crate with nice rust bindings (currently being worked on by [neon](https://github.com/neon-bindings/neon/issues/444)).

Features
========

Different API versions may be selected via feature-flag. See the [N-API Version Matrix](https://nodejs.org/dist/latest-v13.x/docs/api/n-api.html#n_api_n_api_version_matrix) for details.

- `napi_v5` supported by all [actively maintained](https://nodejs.org/en/about/releases/) NodeJS releases
- `napi_v6` requires `v13.7.0`, this is the **default** (if no flag is given)
- the experimental N-API features may be enabled via feature flag `experimental` (*off* by default)


