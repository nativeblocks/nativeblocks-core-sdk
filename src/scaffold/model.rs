//! The scaffold feature's public wire contract.
//!
//! Ports `domain/model/NativeFrameRouteModel.kt` (the scaffold/route models).
//! These are the only types the `Client` trait exposes; everything in `data/`
//! is internal. `serde` derives make them the serializable contract; `uniffi`
//! derives make them ready to cross the FFI boundary as a snapshot.

use serde::{Deserialize, Serialize};

/// The scaffold: the project's route table. Ports `NativeScaffoldModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeScaffoldModel {
    /// All frame routes in the project.
    pub frames: Vec<NativeFrameRouteModel>,
}

/// A single frame route and its metadata. Ports `NativeFrameRouteModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeFrameRouteModel {
    /// Unique identifier of the frame route.
    pub id: Option<String>,
    /// Name of the frame route.
    pub name: Option<String>,
    /// Frame type (FRAME / BOTTOM_SHEET / DIALOG).
    pub frame_type: Option<FrameTypeModel>,
    /// Route path associated with the frame.
    pub route: Option<String>,
    /// Target platform of the frame.
    pub platform: Option<String>,
    /// Arguments declared by the route.
    pub route_arguments: Option<Vec<NativeRouteArgumentsModel>>,
}

/// A single route argument. Ports `NativeRouteArgumentsModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeRouteArgumentsModel {
    /// Name of the route argument.
    pub name: Option<String>,
}

/// Supported frame types. Ports `FrameTypeModel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum FrameTypeModel {
    /// Standard frame.
    Frame,
    /// Bottom sheet.
    BottomSheet,
    /// Dialog.
    Dialog,
}

impl FrameTypeModel {
    /// The wire string for this type.
    pub fn as_str(self) -> &'static str {
        match self {
            FrameTypeModel::Frame => "FRAME",
            FrameTypeModel::BottomSheet => "BOTTOM_SHEET",
            FrameTypeModel::Dialog => "DIALOG",
        }
    }

    /// Parses a wire string, defaulting to [`FrameTypeModel::Frame`] on no match
    /// (matches Kotlin's `fromString`).
    pub fn from_string(value: &str) -> Self {
        match value {
            "BOTTOM_SHEET" => FrameTypeModel::BottomSheet,
            "DIALOG" => FrameTypeModel::Dialog,
            _ => FrameTypeModel::Frame,
        }
    }
}
