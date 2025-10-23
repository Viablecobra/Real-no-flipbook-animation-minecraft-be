fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/android.rs");
    
    // Set soname for shared library
    if std::env::var("TARGET").unwrap().contains("linux") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libnoflipbookanim.so");
    }
    
    // Android specific linking
    if std::env::var("TARGET").unwrap().contains("android") {
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=android");
    }
}