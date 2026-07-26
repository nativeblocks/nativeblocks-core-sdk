use crate::feature::scaffold::data::dto::{FrameRouteDto, RouteArgumentsDto, ScaffoldDataDto};
use crate::feature::scaffold::domain::model::{
    FrameRouteModel, FrameTypeModel, RouteArgumentsModel, ScaffoldModel,
};

pub(super) fn to_model(dto: &ScaffoldDataDto) -> ScaffoldModel {
    let frames = dto
        .scaffold
        .as_ref()
        .map(|s| s.frames.iter().map(map_frame_route).collect())
        .unwrap_or_default();
    return ScaffoldModel { frames };
}

fn map_frame_route(dto: &FrameRouteDto) -> FrameRouteModel {
    return FrameRouteModel {
        id: dto.id.clone(),
        name: dto.name.clone(),
        frame_type: Some(FrameTypeModel::from_string(
            dto.frame_type.as_deref().unwrap_or(""),
        )),
        route: dto.route.clone(),
        platform: dto.platform.clone(),
        route_arguments: dto
            .route_arguments
            .as_ref()
            .map(|args| args.iter().map(map_route_argument).collect()),
    };
}

fn map_route_argument(dto: &RouteArgumentsDto) -> RouteArgumentsModel {
    return RouteArgumentsModel {
        name: dto.name.clone(),
    };
}
