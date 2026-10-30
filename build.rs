fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=jnigraphics");
    }
}