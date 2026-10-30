fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=jnigraphics");
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GLESv3");
    }
}