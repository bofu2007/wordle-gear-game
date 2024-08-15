use session_proxy_io::WordleMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<WordleMetadata>();
}
