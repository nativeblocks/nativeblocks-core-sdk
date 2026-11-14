use serde::Deserialize;

use crate::feature::scaffold::{FrameRouteModel, FrameTypeModel, RouteArgumentsModel};

pub(super) const MANIFEST_FILE: &str = "manifest.json";
pub(super) const MANIFEST_VERSION: u32 = 1;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EmbeddedManifest {
    pub(super) version: u32,
    pub(super) project_id: Option<String>,
    pub(super) platform: String,
    #[serde(default)]
    pub(super) frames: Vec<FrameEntry>,
    #[serde(default)]
    pub(super) localizations: Vec<LocalizationEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FrameEntry {
    pub(super) id: Option<String>,
    pub(super) name: Option<String>,
    #[serde(rename = "type")]
    pub(super) frame_type: Option<String>,
    pub(super) route: String,
    pub(super) platform: Option<String>,
    pub(super) route_arguments: Option<serde_json::Value>,
    pub(super) checksum: String,
    pub(super) file: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LocalizationEntry {
    pub(super) language_code: String,
    pub(super) checksum: String,
    pub(super) file: String,
}

pub(super) fn parse(
    bytes: &[u8],
    project_id: &str,
    platform: &str,
) -> Result<EmbeddedManifest, String> {
    let manifest: EmbeddedManifest =
        serde_json::from_slice(bytes).map_err(|error| format!("invalid manifest: {error}"))?;
    if manifest.version != MANIFEST_VERSION {
        return Err(format!("unsupported manifest version {}", manifest.version));
    }
    if manifest.project_id.as_deref() != Some(project_id) {
        return Err(format!(
            "manifest is for project {}, this instance is {project_id}",
            manifest.project_id.as_deref().unwrap_or("unknown")
        ));
    }
    if !manifest.platform.eq_ignore_ascii_case(platform) {
        return Err(format!(
            "manifest is for {}, this app is {platform}",
            manifest.platform
        ));
    }
    return Ok(manifest);
}

pub(super) fn is_safe_name(name: &str) -> bool {
    return !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
}

pub(super) fn is_safe_path(path: &str) -> bool {
    return !path.is_empty()
        && !path.starts_with('/')
        && !path.split(['/', '\\']).any(|part| part == "..");
}

pub(super) fn to_frame_route(entry: &FrameEntry) -> FrameRouteModel {
    return FrameRouteModel {
        id: entry.id.clone(),
        name: entry.name.clone(),
        frame_type: Some(FrameTypeModel::from_string(
            entry.frame_type.as_deref().unwrap_or(""),
        )),
        route: Some(entry.route.clone()),
        platform: entry.platform.clone(),
        route_arguments: entry.route_arguments.as_ref().and_then(route_arguments),
    };
}

fn route_arguments(value: &serde_json::Value) -> Option<Vec<RouteArgumentsModel>> {
    let arguments = value.as_array()?;
    return Some(
        arguments
            .iter()
            .map(|argument| RouteArgumentsModel {
                name: argument
                    .get("name")
                    .and_then(|name| name.as_str())
                    .map(str::to_string),
            })
            .collect(),
    );
}
