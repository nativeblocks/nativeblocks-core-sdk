use super::dto::{NativeFrameRouteDto, NativeRouteArgumentsDto, NativeScaffoldDataDto};
use crate::scaffold::domain::model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

impl NativeScaffoldDataDto {
    pub(crate) fn to_model(&self) -> NativeScaffoldModel {
        let frames = self
            .scaffold
            .as_ref()
            .map(|s| s.frames.iter().map(map_frame_route).collect())
            .unwrap_or_default();
        NativeScaffoldModel { frames }
    }
}

fn map_frame_route(dto: &NativeFrameRouteDto) -> NativeFrameRouteModel {
    NativeFrameRouteModel {
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
    }
}

fn map_route_argument(dto: &NativeRouteArgumentsDto) -> NativeRouteArgumentsModel {
    NativeRouteArgumentsModel {
        name: dto.name.clone(),
    }
}
