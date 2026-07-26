use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BaseDto<D> {
    #[serde(default = "none")]
    pub data: Option<D>,
    #[serde(default)]
    pub errors: Option<Vec<BaseErrorDto>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BaseErrorDto {
    pub message: String,
    pub extensions: BaseErrorClassificationDto,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BaseErrorClassificationDto {
    pub classification: String,
}

fn none<D>() -> Option<D> {
    None
}
