fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        unsafe {
            std::env::set_var("WINDRES", "x86_64-w64-mingw32-windres");
        }
        
        embed_resource::compile("mpfree.rc", embed_resource::NONE);
    }
}