/// Scroll payload attributes consumed directly by the reth v2 payload builder.
///
/// Reth v2 removed the separate builder-attributes associated type. Keep this alias so downstream
/// Scroll code using the former name continues to compile while sharing the Engine API type.
pub type ScrollPayloadBuilderAttributes = scroll_alloy_rpc_types_engine::ScrollPayloadAttributes;
