fn main () {
    #[cfg(target_arch="x86_64")]
    {
        println!("cargo:rustc-link-search=arch/x86_64-linux");
    }

    #[cfg(target_arch="arm")]
    {
        println! ("cargo:rustc-link-search=arch/arm-linux");
    }

    #[cfg(target_arch="aarch64")]
    {
        println! ("cargo:rustc-link-search=arch/aarch64-macos");
    }

    println! ("cargo:rustc-link-search=arch/x86_64-linux");
    println! ("cargo:rustc-flags=-latomic");
}