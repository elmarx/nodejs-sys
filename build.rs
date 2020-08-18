use bindgen::Builder;
use bindgen::EnumVariation;
use std::env;
use std::path::PathBuf;

trait ExtendedBuilder {
    fn set_experimental(self) -> Self;
    fn set_napi_version(self) -> Self;
}

impl ExtendedBuilder for Builder {
    fn set_experimental(self) -> Builder {
        if cfg!(feature = "experimental") {
            self.clang_arg("-D NAPI_EXPERIMENTAL")
        } else {
            self
        }
    }

    fn set_napi_version(self) -> Builder {
        if cfg!(feature = "napi_v6") {
            self.clang_arg("-D NAPI_VERSION=6")
        } else if cfg!(feature = "napi_v5") {
            self.clang_arg("-D NAPI_VERSION=5")
        } else {
            self
        }
    }
}

fn main() {
    // taken from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    // allow to set the location of node_api.h
    let bindings = bindgen::Builder::default()
        .set_experimental()
        .set_napi_version()
        .header("node/src/node_api.h")
        .whitelist_function("napi_.*")
        .whitelist_type("napi_.*")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .bitfield_enum("napi_key_filter")
        .bitfield_enum("napi_property_attributes")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
