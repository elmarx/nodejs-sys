use bindgen::Builder;
use std::env;
use std::path::PathBuf;

trait ExtendedBuilder {
    fn set_experimental(self) -> Self;
    fn set_include(self) -> Self;
}

impl ExtendedBuilder for Builder {
    fn set_experimental(self) -> Builder {
        if cfg!(feature = "experimental") {
            self.clang_arg("-D NAPI_EXPERIMENTAL")
        } else {
            self
        }
    }

    fn set_include(self) -> Builder {
        match option_env!("NODE_INCLUDE") {
            Some(node_include_path) => self.clang_arg(format!("-I{}", node_include_path)),
            None => self,
        }
    }
}

fn main() {
    // taken from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    // allow to set the location of node_api.h
    let bindings = bindgen::Builder::default()
        .set_include()
        .set_experimental()
        .header("wrapper.h")
        .whitelist_function("napi_.*")
        .whitelist_type("napi_.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
