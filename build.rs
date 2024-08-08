// build.rs
use pebbles_game_io::PebblesMetadata; // 引入PebblesMetadata结构体

fn main() {
    // 使用gear_wasm_builder构建智能合约，并附加元数据
    gear_wasm_builder::build_with_metadata::<PebblesMetadata>();
}
