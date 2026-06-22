use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct NativeScaffoldDataDto {
    pub(super) scaffold: Option<NativeScaffoldDto>,
}

#[derive(Deserialize)]
pub(super) struct NativeScaffoldDto {
    #[serde(default)]
    pub(super) frames: Vec<NativeFrameRouteDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeFrameRouteDto {
    pub(super) id: Option<String>,
    pub(super) name: Option<String>,
    #[serde(rename = "type")]
    pub(super) frame_type: Option<String>,
    pub(super) route: Option<String>,
    pub(super) platform: Option<String>,
    pub(super) route_arguments: Option<Vec<NativeRouteArgumentsDto>>,
}

#[derive(Deserialize)]
pub(super) struct NativeRouteArgumentsDto {
    pub(super) name: Option<String>,
}
