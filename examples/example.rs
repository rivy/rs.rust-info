// spell-checker:ignore (acronyms) LLVM
extern crate rust_info;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rust_info = if args.len() < 2 {
        rust_info::get()
    } else {
        rust_info::get_for_target(&args[1])
    };

    // println!("{:?}", rust_info);

    println!("Version: {}", rust_info.version.unwrap());
    println!("Channel: {:#?}", rust_info.channel.unwrap());
    println!("Binary: {}", rust_info.binary.unwrap());
    println!("Commit-date: {}", rust_info.commit_date.unwrap());
    println!("Commit-hash: {}", rust_info.commit_hash.unwrap());
    println!("Host: {}", rust_info.host.unwrap());
    println!("Release: {}", rust_info.release.unwrap());
    println!("LLVM version: {}", rust_info.llvm_version.unwrap());
    println!(
        "Target Arch: {}",
        rust_info.target_arch.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Endian: {}",
        rust_info.target_endian.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Env: {}",
        rust_info.target_env.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Family: {}",
        rust_info.target_family.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Features: {:?}",
        rust_info.target_features.unwrap_or([].to_vec())
    );
    println!(
        "Target OS: {}",
        rust_info.target_os.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Pointer Width: {}",
        rust_info
            .target_pointer_width
            .unwrap_or("unknown".to_string())
    );
    println!(
        "Target Vendor: {}",
        rust_info.target_vendor.unwrap_or("unknown".to_string())
    );
    println!(
        "Target Triple: {}",
        rust_info.target_triple.unwrap_or("unknown".to_string())
    );
}
