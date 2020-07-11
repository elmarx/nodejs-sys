//! Low level bindings to nodejs' [N-API](https://nodejs.org/dist/latest-v13.x/docs/api/n-api.html)
//!
//!
//! # Features
//!
//! Different API versions may be selected via feature-flag. See the [N-API Version Matrix](https://nodejs.org/dist/latest-v14.x/docs/api/n-api.html#n_api_n_api_version_matrix) for details.
//!
//! - `napi_v5` supported by all [actively maintained](https://nodejs.org/en/about/releases/) NodeJS releases
//! - `napi_v6` requires at least `v13.7.0`, this is the **default** (if no flag is given)
//! - the experimental N-API features may be enabled via feature flag `experimental` (*off* by default)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
