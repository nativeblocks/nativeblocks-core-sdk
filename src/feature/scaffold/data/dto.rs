use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ScaffoldDataDto {
    pub(super) scaffold: Option<ScaffoldDto>,
}

#[derive(Deserialize)]
pub(super) struct ScaffoldDto {
    #[serde(default)]
    pub(super) frames: Vec<FrameRouteDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FrameRouteDto {
    pub(super) id: Option<String>,
    pub(super) name: Option<String>,
    #[serde(rename = "type")]
    pub(super) frame_type: Option<String>,
    pub(super) route: Option<String>,
    pub(super) platform: Option<String>,
    pub(super) route_arguments: Option<Vec<RouteArgumentsDto>>,
}

#[derive(Deserialize)]
pub(super) struct RouteArgumentsDto {
    pub(super) name: Option<String>,
}
