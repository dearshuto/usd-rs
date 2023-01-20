fn main() {
    // cmake による tinyusdz のビルド
    let dst = cmake::Config::new("ffi/Externals/tinyusdz")
        // .always_configure(false)
        // .very_verbose(true)
        .configure_arg("-DTINYUSDZ_BUILD_EXAMPLES=OFF")
        .configure_arg("-DTINYUSDZ_BUILD_SHARED_LIBS=OFF")
        .configure_arg("-DTINYUSDZ_BUILD_TESTS=OFF")
        .configure_arg("-DTINYUSDZ_WITH_C_API=OFF")
        .configure_arg("-DTINYUSDZ_WITH_TOOL_USDA_PARSER=OFF")
        .configure_arg("-DTINYUSDZ_WITH_TOOL_USDC_PARSER=OFF")
        .build_target("tinyusdz_static")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=tinyusdz_static");

    // ffi ライブラリのビルド
    let dst = cmake::Config::new("ffi")
        // .always_configure(false)
        // .very_verbose(true)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=usdffi");
    println!("cargo:rustc-link-lib=c++");
}
