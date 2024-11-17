//!
#![warn(missing_debug_implementations, missing_docs)]
///
// pub mod error;
///
// pub mod file_utils;
///
// pub mod prelude;
///
// pub mod scanner;
// mod syntax;
// mod tests;
///
// pub mod transpiler;
use crate::transpiler::decompiler::Decompiler;

use std::fs;
use std::io::prelude::*;
use std::path;

/// Compiles a set of Rattle code into a Python script.
pub fn compile(code: String, path: String) -> Result<String, ()> {
    let compiler = Decompiler::new(&code);
    match compiler {
        Ok(mut compiler_) => {
            match compiler_.decompile() {
                Ok(_) => (),
                Err(msg) => return Ok(format!("Failed: {}", msg)),
            }

            compiler_
                .py
                .push_str("\nif __name__ == '__main__':\n\tstart([])");

            if &path[path.len() - 7..] != ".rattle" {
                return Ok("Failed: Rattle files must end in .rattle".to_string());
            }

            let py_fn = format!("../{}.py", &path[..path.len() - 7]);
            // let py_fn = format!("{}.py", &path[..path.len() - 7]);
            let py_path = path::Path::new(&py_fn);

            let mut file = match fs::File::create(&py_path) {
                Err(why) => panic!("Couldn't create {}: {}", py_fn, why),
                Ok(file_) => file_,
            };

            match file.write_all(compiler_.py.as_bytes()) {
                Err(why) => return Ok(format!("Failed: Could not write to {}: {}", py_fn, why)),
                Ok(_) => return Ok("Successful".to_string()),
            }
        }
        Err(msg) => return Ok(format!("Failed: {}", msg)),
    }
}
