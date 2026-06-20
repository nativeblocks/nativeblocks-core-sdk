use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeScaffoldModel {
    pub frames: Vec<NativeFrameRouteModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeFrameRouteModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub frame_type: Option<FrameTypeModel>,
    pub route: Option<String>,
    pub platform: Option<String>,
    pub route_arguments: Option<Vec<NativeRouteArgumentsModel>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeRouteArgumentsModel {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum FrameTypeModel {
    Frame,
    BottomSheet,
    Dialog,
}

impl FrameTypeModel {
    pub fn as_str(self) -> &'static str {
        match self {
            FrameTypeModel::Frame => "FRAME",
            FrameTypeModel::BottomSheet => "BOTTOM_SHEET",
            FrameTypeModel::Dialog => "DIALOG",
        }
    }

    pub fn from_string(value: &str) -> Self {
        match value {
            "BOTTOM_SHEET" => FrameTypeModel::BottomSheet,
            "DIALOG" => FrameTypeModel::Dialog,
            _ => FrameTypeModel::Frame,
        }
    }
}
