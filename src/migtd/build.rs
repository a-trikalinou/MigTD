// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::env;
use std::process::Command;

fn main() {
    // Check if we're running tests using the CARGO_CFG_TEST environment variable
    let is_test = env::var("CARGO_CFG_TEST").is_ok();


//    println!("cargo:rustc-link-arg=-defsym=__ImageBase=0");
    if is_test {
        println!("cargo:rustc-link-arg=-Wl,--defsym,__ImageBase=0");
    }
    else {
        println!("cargo:rustc-link-arg=-defsym=__ImageBase=0");
    }
//    println!("cargo:rustc-link-arg=-Wl,-defsym=__ImageBase=0");


    // // Get the path to the attestation library
    // let crate_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // let lib_path = crate_path
    //     .join("../deps/linux-sgx")
    //     .display()
    //     .to_string();

    // // Add the attestation library path and linking information
    // let search_dir = format!(
    //     "{}/external/dcap_source/QuoteGeneration/quote_wrapper/servtd_attest/linux",
    //     &lib_path
    // );

    // // Debug print to verify paths
    // eprintln!("Looking for library in: {}", search_dir);

    // println!("cargo:rustc-link-search=native={}", search_dir);
    // println!("cargo:rustc-link-lib=static=servtd_attest");





    // Skip this, since it's done in the attestation library build.rs
    // let _ = env::var("DEBUG").ok().map(|_| env::remove_var("DEBUG"));

    // // Unset the CC and AR variable
    // let _ = env::var("CC").ok().map(|_| env::remove_var("CC"));
    // let _ = env::var("AR").ok().map(|_| env::remove_var("AR"));

    // let crate_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // let lib_path = crate_path
    //     .join("../../deps/linux-sgx")
    //     .display()
    //     .to_string();

    // // make servtd_attest_preparation
    // let status = Command::new("make")
    //     .args(["-C", &lib_path, "servtd_attest_preparation"])
    //     .status()
    //     .expect("failed to run make servtd_attest_preparation for attestation library!");
    // assert!(
    //     status.success(),
    //     "failed to build servtd_attest_preparation: {status}"
    // );

    // // make servtd_attest
    // let status = Command::new("make")
    //     .args(["-C", &lib_path, "servtd_attest"])
    //     .status()
    //     .expect("failed to run make servtd_attest for attestation library!");
    // assert!(status.success(), "failed to build servtd_attest: {status}");

    // let search_dir = format!(
    //     "{}/external/dcap_source/QuoteGeneration/quote_wrapper/servtd_attest/linux",
    //     &lib_path
    // );

    // println!("cargo:rustc-link-search=native={}", search_dir);
    // println!("cargo:rustc-link-lib=static=servtd_attest");
}
