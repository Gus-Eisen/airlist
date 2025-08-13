fn main() {
    #[cfg(target_arch = "wasm32")]
    main::maverick_main();
}
