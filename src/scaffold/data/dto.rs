use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NativeScaffoldDataDto {
    pub(crate) scaffold: Option<NativeScaffoldDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NativeScaffoldDto {
    #[serde(default)]
    pub(crate) frames: Vec<NativeFrameRouteDto>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeFrameRouteDto {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) frame_type: Option<String>,
    pub(crate) route: Option<String>,
    pub(crate) platform: Option<String>,
    pub(crate) route_arguments: Option<Vec<NativeRouteArgumentsDto>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NativeRouteArgumentsDto {
    pub(crate) name: Option<String>,
}
