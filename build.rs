/*
    pbskids-dl
    Copyright (C) 2024 The pbskids-dl Team

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

fn main() {
    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=src/ui.fl");
    let generator = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    generator.in_out("src/ui.fl", out_path.join("ui.rs").to_str().unwrap())
        .expect("Failed to generate rust from fl file!");
}
