use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BaseDto<D> {
    #[serde(default = "none")]
    pub data: Option<D>,
    #[serde(default)]
    pub errors: Option<Vec<BaseErrorDto>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseErrorDto {
    pub message: String,
    pub extensions: BaseErrorClassificationDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseErrorClassificationDto {
    pub classification: String,
}

fn none<D>() -> Option<D> {
    None
}
