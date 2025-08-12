fn main() {
    #[cfg(all(target_arch = "wasm32"))]
    main::maverick_main();
}
