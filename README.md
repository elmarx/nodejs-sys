nodejs-sys
----------

Bindings for NodeJS' [N-API](https://nodejs.org/dist/latest-v12.x/docs/api/n-api.html).

Requirements
============

To build this crate you need `libclang`, since bindings are being generated at build-time (by [bindgen](https://docs.rs/bindgen/)).

The experimental N-API features may be enabled via usual cargo feature flag "experimental".

Stability
=========

This crate is probably already done/stable, the real work is to provide a wrapper crate with nice rust bindings (currently being worked on by [neon](https://github.com/neon-bindings/neon/issues/444)).
