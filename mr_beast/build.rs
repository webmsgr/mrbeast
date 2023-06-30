fn main() {
    #[cfg(windows)]
    embed_resource::compile("rec.rc", std::iter::empty::<&str>());
}
