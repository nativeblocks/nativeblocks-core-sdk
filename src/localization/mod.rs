mod data;
pub(crate) mod di;
mod domain;
mod presenter;

pub use domain::model::NativeLocalizationModel;
pub use presenter::{
    LocalizationClient, LocalizationState, LocalizationStateManager, LocalizationStateObserver,
};
pub(crate) use presenter::dispose;

pub(crate) use domain::repository::LocalizationRepository;
