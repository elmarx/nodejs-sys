nodejs-sys
----------

Bindings for NodeJS' [N-API](https://nodejs.org/dist/latest-v12.x/docs/api/n-api.html).

Requirements
============

To build this create you need `libclang` and NodeJS' *node_api.h* available, since bindings are being generated at build-time.

If *node_api.h* is not discoverable via normal resolution of includes, set environment variable
 `NODE_INCLUDE=/path/to/node-v12.1.0-linux-x64/include/node`.
 
The experimental N-API features may be enabled via usual cargo/rust feature-flags.

Stability
=========

This crate is probably already done/stable, the real work is a to provide a wrapper crate with nice rust bindingsj
