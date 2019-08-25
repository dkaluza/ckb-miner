fn main() {
    cc::Build::new()
        .file("src/worker/include/eaglesong.c")
        .include("src/worker/include")
        .flag("-O3")
        .flag("-lcrypto")
        .static_flag(true)
        .compile("libeaglesong.a.0");

    cc::Build::new()
        .file("src/worker/include/eaglesong_avx2.c")
        .include("src/worker/include")
        .flag("-O3")
        .flag("-lcrypto")
        .flag("-mavx2")
        .static_flag(true)
        .compile("libeaglesong.a.1");

    cc::Build::new()
        .file("src/worker/include/eaglesong_avx512.c")
        .include("src/worker/include")
        .flag("-O3")
        .flag("-lcrypto")
        .flag("-mavx512f")
        .static_flag(true)
        .compile("libeaglesong.a.2");

    #[cfg(feature = "gpu")]
    cc::Build::new()
        .file("src/worker/include/eaglesong.cu")
        .include("src/worker/include")
        .flag("-O3")
        .flag("-lcrypto")
        .cuda(true)
        .compile("libeaglesong.a.3");

    // Add link directory
    // - This path depends on where you install CUDA (i.e. depends on your Linux distribution)
    // - This should be set by `$LIBRARY_PATH`
    #[cfg(feature = "gpu")]
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    #[cfg(feature = "gpu")]
    println!("cargo:rustc-link-lib=cudart");
}
