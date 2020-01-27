#![allow(deprecated,unknown_lints)]
#![cfg_attr(any(feature="feat_test_pedantic"), warn(unknown_lints))]
#![cfg_attr(any(feature="feat_test_pedantic"), deny(deprecated))]
// known lints (as of 1.42.0-nightly)
#![cfg_attr(all(feature="nightly",feature="feat_test_pedantic"), deny(
    array_into_iter,
    bindings_with_variant_name,
    overlapping_patterns,
    uncommon_codepoints,
))]
// known lints (as of v1.39.0)
#![cfg_attr(any(feature="feat_test_pedantic"), deny(
    ambiguous_associated_items,
    anonymous_parameters,
    deprecated_in_future,
    exported_private_dependencies,
    ill_formed_attribute_input,
    incomplete_features,
    indirect_structural_match,
    invalid_value,
    meta_variable_misuse,
    mutable_borrow_reservation_conflict,
    non_ascii_idents,
    order_dependent_trait_objects,
    private_doc_tests,
    redundant_semicolon,
    soft_unstable,
))]
// known lints (as of v1.31.0)
#![deny(
    absolute_paths_not_starting_with_crate,
    const_err,
    dead_code,
    deprecated,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    invalid_type_param_default,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # rust_info
//!
//! Extracts and provides the current rust compiler information.
//!
//! This library main goal is to provide development/build tools such as
//! [cargo-make](https://sagiegurari.github.io/cargo-make/)the needed information on the current rust installation and
//! setup.
//!
//! # Examples
//!
//! ```
//! extern crate rust_info;
//!
//! fn main() {
//!     let rust_info = rust_info::get();
//!
//!     println!("Version: {}", rust_info.version.unwrap());
//!     println!("Channel: {:#?}", rust_info.channel.unwrap());
//!     println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
//!     println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
//!     println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
//!     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
//!     println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
//!     println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! rust_info = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/rust_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/rust_info/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

mod rustinfo;
pub mod types;

use crate::types::RustInfo;

/// Loads and returns the current rust compiler version and setup.<br>
/// In case partial data is not available, those values will be set to Option::None.
///
/// # Example
///
/// ```
/// extern crate rust_info;
///
/// fn main() {
///     let rust_info = rust_info::get();
///
///     println!("Version: {}", rust_info.version.unwrap());
///     println!("Channel: {:#?}", rust_info.channel.unwrap());
///     println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
///     println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
///     println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
///     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
///     println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
///     println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
/// }
/// ```
pub fn get() -> RustInfo {
    rustinfo::get_for_target("")
}
/// ...
pub fn get_for_target(target: &str) -> RustInfo {
    rustinfo::get_for_target(target)
}
