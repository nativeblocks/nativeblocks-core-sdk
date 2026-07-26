// //! Off-by-default profiling harness for the warm frame-load hot path.
// //!
// //! Exercises exactly what a warm start pays per frame: rkyv `from_bytes`
// //! (db decode) -> `InternalState::ready` (COW view build) -> `to_frame_full`
// //! (single owned materialization feeding the FFI). Compiled only under `cfg(test)`.
// //!
// //! Flamegraph (macOS needs sudo for dtrace):   ./scripts/flamegraph.sh
// //! Textual phase report (no root required):
// //!   cargo test --profile profiling \
// //!     feature::frame::presenter::state_manager::profiling::heavy_load \
// //!     -- --ignored --nocapture
//
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::time::{Duration, Instant};
//
// use rkyv::rancor::Error as RkyvError;
// use rkyv::util::AlignedVec;
//
// use super::state::InternalState;
// use crate::feature::frame::domain::model::{
//     NativeActionModel, NativeBlockDataModel, NativeBlockModel, NativeBlockPropertyModel,
//     NativeBlockSlotModel, NativeFrameModel, NativeVariableModel,
// };
//
// const SLOTS: [&str; 3] = ["content", "header", "footer"];
// const BRANCHING: usize = 6;
//
// /// Builds a synthetic frame with realistic per-block field density (data,
// /// properties, slots) and a branching parent/slot tree.
// fn build_frame(block_count: usize) -> NativeFrameModel {
//     let mut blocks = HashMap::with_capacity(block_count);
//     for i in 0..block_count {
//         let key = format!("block-{i}");
//         let parent_index = i.saturating_sub(1) / BRANCHING;
//         let parent_key = if i == 0 {
//             String::new()
//         } else {
//             format!("block-{parent_index}")
//         };
//         let data = (0..3)
//             .map(|d| {
//                 let k = format!("data-{d}");
//                 let model = NativeBlockDataModel {
//                     key: k.clone(),
//                     value: format!("value-{i}-{d}"),
//                     data_type: "STRING".to_string(),
//                 };
//                 (k, model)
//             })
//             .collect();
//         let properties = (0..4)
//             .map(|p| {
//                 let k = format!("prop-{p}");
//                 let model = NativeBlockPropertyModel {
//                     key: k.clone(),
//                     value_mobile: format!("m-{i}-{p}"),
//                     value_tablet: format!("t-{i}-{p}"),
//                     value_desktop: format!("d-{i}-{p}"),
//                     property_type: "STRING".to_string(),
//                 };
//                 (k, model)
//             })
//             .collect();
//         let slots = SLOTS
//             .iter()
//             .map(|slot| {
//                 (
//                     slot.to_string(),
//                     NativeBlockSlotModel {
//                         slot: slot.to_string(),
//                     },
//                 )
//             })
//             .collect();
//         blocks.insert(
//             key.clone(),
//             NativeBlockModel {
//                 id: format!("id-{i}"),
//                 parent_id: if i == 0 {
//                     String::new()
//                 } else {
//                     format!("id-{parent_index}")
//                 },
//                 parent_key,
//                 version: 1,
//                 slot: SLOTS[i % SLOTS.len()].to_string(),
//                 key_type: "NATIVE_COLUMN".to_string(),
//                 key,
//                 visibility: "VISIBLE".to_string(),
//                 position: (i % BRANCHING) as i32,
//                 data,
//                 properties,
//                 slots,
//             },
//         );
//     }
//
//     let variables = (0..20)
//         .map(|v| {
//             let k = format!("var-{v}");
//             let model = NativeVariableModel {
//                 key: k.clone(),
//                 value: format!("val-{v}"),
//                 variable_type: "STRING".to_string(),
//             };
//             (k, model)
//         })
//         .collect();
//
//     let actions = (0..10)
//         .map(|a| {
//             let k = format!("block-{a}");
//             let action = NativeActionModel {
//                 id: format!("action-{a}"),
//                 key: k.clone(),
//                 event: "onClick".to_string(),
//                 triggers: Vec::new(),
//             };
//             (k, vec![action])
//         })
//         .collect();
//
//     return NativeFrameModel {
//         checksum: Some("checksum-0".to_string()),
//         variables,
//         blocks,
//         root_key: Some("block-0".to_string()),
//         actions,
//     };
// }
//
// fn env_usize(key: &str, default: usize) -> usize {
//     return std::env::var(key)
//         .ok()
//         .and_then(|value| value.parse().ok())
//         .unwrap_or(default);
// }
//
// #[test]
// #[ignore = "profiling harness: run explicitly (see module docs / scripts/flamegraph.sh)"]
// fn heavy_load() {
//     let block_count = env_usize("NB_PROFILE_BLOCKS", 504);
//     let iterations = env_usize("NB_PROFILE_ITERS", 4000);
//
//     let frame = build_frame(block_count);
//     let bytes = rkyv::to_bytes::<RkyvError>(&frame)
//         .expect("serialize frame")
//         .to_vec();
//
//     let args: HashMap<String, String> = HashMap::new();
//     let globals: HashMap<String, String> = HashMap::new();
//
//     let mut decode = Duration::ZERO;
//     let mut ready = Duration::ZERO;
//     let mut emit = Duration::ZERO;
//     let mut sink = 0usize;
//
//     for _ in 0..iterations {
//         let mut aligned = AlignedVec::<16>::new();
//         aligned.extend_from_slice(&bytes);
//
//         let clock = Instant::now();
//         let decoded = rkyv::from_bytes::<NativeFrameModel, RkyvError>(aligned.as_slice())
//             .expect("decode frame");
//         decode += clock.elapsed();
//
//         let base = Arc::new(decoded);
//
//         let clock = Instant::now();
//         let state = InternalState::ready(base, &args, &globals);
//         ready += clock.elapsed();
//
//         let clock = Instant::now();
//         let full = state.to_frame_full();
//         emit += clock.elapsed();
//
//         sink = sink.wrapping_add(full.blocks.len() + full.block_sub_keys.len());
//     }
//
//     let total = decode + ready + emit;
//     let per = |value: Duration| value.as_secs_f64() * 1e6 / iterations as f64;
//     let share = |value: Duration| 100.0 * value.as_secs_f64() / total.as_secs_f64();
//
//     eprintln!("\n=== warm frame-load profile ===");
//     eprintln!("blocks / frame     : {block_count}");
//     eprintln!("iterations         : {iterations}");
//     eprintln!("serialized size    : {} KiB", bytes.len() / 1024);
//     eprintln!("checksum (sink)     : {sink}");
//     eprintln!("\nphase                    per-load(us)     share");
//     eprintln!(
//         "rkyv from_bytes        : {:>10.2}     {:>5.1}%",
//         per(decode),
//         share(decode)
//     );
//     eprintln!(
//         "InternalState::ready   : {:>10.2}     {:>5.1}%",
//         per(ready),
//         share(ready)
//     );
//     eprintln!(
//         "to_frame_full (emit)   : {:>10.2}     {:>5.1}%",
//         per(emit),
//         share(emit)
//     );
//     eprintln!("-----------------------------------------------");
//     eprintln!("total per warm load    : {:>10.2}     100.0%", per(total));
// }
//
// #[cfg(test)]
// mod profiling;
