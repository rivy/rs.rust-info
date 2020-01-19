//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use crate::types::{RustChannel, RustInfo};
use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitStatus};

/// Returns the exit code (-1 if no exit code found)
fn get_exit_code(exit_status: Result<ExitStatus, Error>) -> i32 {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => value,
                    None => -1,
                }
            } else {
                0
            }
        }
        _ => -1,
    }
}

fn load_version(rust_info: &mut RustInfo, rust_exe: &str) {
    let result = Command::new(rust_exe).arg("--verbose").arg("--version").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let mut values = HashMap::<String, String>::new();

                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    if line.contains(":") {
                        let parts: Vec<&str> = line.split(':').collect();
                        values.insert(parts[0].to_string(), parts[1].trim().to_string());
                    } else {
                        let parts: Vec<&str> = line.split(' ').collect();

                        if (parts.len() >= 3) && (parts[0] == "rustc") {
                            let version_part = parts[1];

                            let version_parts: Vec<&str> = version_part.split('-').collect();

                            if version_parts.len() > 0 {
                                rust_info.version = Some(version_parts[0].to_string());

                                if version_parts.len() == 1 {
                                    rust_info.channel = Some(RustChannel::Stable);
                                } else if version_parts[1].contains("beta") {
                                    rust_info.channel = Some(RustChannel::Beta);
                                } else if version_parts[1].contains("nightly") {
                                    rust_info.channel = Some(RustChannel::Nightly);
                                }
                            }
                        }
                    }
                }

                let mut value = values.remove("binary");
                if value.is_some() {
                    rust_info.binary = Some(value.unwrap());
                }

                value = values.remove("commit-date");
                if value.is_some() {
                    rust_info.commit_date = Some(value.unwrap());
                }

                value = values.remove("commit-hash");
                if value.is_some() {
                    rust_info.commit_hash = Some(value.unwrap());
                }

                value = values.remove("host");
                if value.is_some() {
                    rust_info.host = Some(value.unwrap());
                }

                value = values.remove("release");
                if value.is_some() {
                    rust_info.release = Some(value.unwrap());
                }

                value = values.remove("LLVM version");
                if value.is_some() {
                    rust_info.llvm_version = Some(value.unwrap());
                }
            }
        }
        _ => (),
    };
}

fn load_setup(rust_info: &mut RustInfo, rust_exe: &str, target: &str) {
    let result = if target == "" {
        Command::new(rust_exe).arg("--print").arg("cfg").output()
    } else {
        Command::new(rust_exe).arg("--target").arg(target).arg("--print").arg("cfg").output()
    };

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let mut values = HashMap::<String, Vec<String>>::new();

                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    if line.contains("=") {
                        let parts: Vec<&str> = line.split('=').collect();
                        let value = str::replace(parts[1], "\"", "");
                        if let Some(v) = values.get_mut(&parts[0].to_string()) {
                            v.push(value.to_string());
                        } else {
                            let mut v = Vec::<String>::new();
                            v.push(value.to_string());
                            values.insert(parts[0].to_string(), v.to_vec());
                        }
                    }
                }

                let mut value = values.remove("target_arch");
                if value.is_some() {
                    rust_info.target_arch = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_endian");
                if value.is_some() {
                    rust_info.target_endian = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_env");
                if value.is_some() {
                    rust_info.target_env = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_family");
                if value.is_some() {
                    rust_info.target_family = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_feature");
                if value.is_some() {
                    rust_info.target_features = Some(value.unwrap());
                }

                value = values.remove("target_os");
                if value.is_some() {
                    rust_info.target_os = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_pointer_width");
                if value.is_some() {
                    rust_info.target_pointer_width = Some(((value.unwrap())[0]).to_string());
                }

                value = values.remove("target_vendor");
                if value.is_some() {
                    rust_info.target_vendor = Some(((value.unwrap())[0]).to_string());
                }
            }
        }
        _ => (),
    };
}

fn calculate_triple(rust_info: &mut RustInfo) {
    if let (Some(arch), Some(os)) = (&rust_info.target_arch, &rust_info.target_os) {
        let mut triple = String::new();

        let mut s = arch.to_string();
        if arch == "x86" {
            s = "i586".to_string();
            if let Some(features) = &rust_info.target_features {
                if features.contains(&"sse".to_string()) || features.contains(&"sse2".to_string()) {
                    s = "i686".to_string();
                }
            }
        }
        triple.push_str(&s);

        if let Some(vendor) = &rust_info.target_vendor {
            triple.push_str("-");
            triple.push_str(vendor);
        }

        triple.push_str("-");
        triple.push_str(os);

        if let Some(env) = &rust_info.target_env {
            triple.push_str("-");
            triple.push_str(env);
        }

        rust_info.target_triple = Some(triple);
    }
}

/// Loads and returns the current rust compiler version and setup.<br>
/// In case partial data is not available, those values will be set to Option::None.
pub(crate) fn get_for_target(target: &str) -> RustInfo {
    let mut rust_info = RustInfo::new();
    let rust_exe = std::env::var("RUSTC").unwrap_or("rustc".to_string());

    load_version(&mut rust_info, &rust_exe);

    load_setup(&mut rust_info, &rust_exe, target);

    calculate_triple(&mut rust_info);

    rust_info
}
