fn main() {
    let dst = zigcli::build("zig_package");
    let dst_lib = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", dst_lib.display());
    println!("cargo:rustc-link-lib=static=zig_package");
}
