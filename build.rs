fn main() {
    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-search=lib/x86_64-linux");
        }
    }

    #[cfg(target_arch = "arm")]
    {
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-search=lib/arm-linux");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-search=lib/aarch64-macos");
        }
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-search=lib/aarch64-linux");
        }
    }

    println!("cargo:rustc-flags=-latomic");
}
