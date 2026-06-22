use crate::scaffold::dto::{NativeFrameRouteDto, NativeRouteArgumentsDto, NativeScaffoldDataDto};
use crate::scaffold::model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

pub(super) fn to_model(dto: &NativeScaffoldDataDto) -> NativeScaffoldModel {
    let frames = dto
        .scaffold
        .as_ref()
        .map(|s| s.frames.iter().map(map_frame_route).collect())
        .unwrap_or_default();
    return NativeScaffoldModel { frames };
}

fn map_frame_route(dto: &NativeFrameRouteDto) -> NativeFrameRouteModel {
    return NativeFrameRouteModel {
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

fn map_route_argument(dto: &NativeRouteArgumentsDto) -> NativeRouteArgumentsModel {
    return NativeRouteArgumentsModel {
        name: dto.name.clone(),
    };
}
