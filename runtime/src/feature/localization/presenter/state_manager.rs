use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use tokio::task::JoinHandle;

use crate::feature::localization::domain::repository::LocalizationRepository;
use crate::library::result::ErrorType;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum LocalizationState {
    Loading {},
    Ready {},
    Error { message: String },
}

#[uniffi::export(with_foreign)]
pub trait LocalizationStateObserver: Send + Sync {
    fn on_state_changed(&self, state: LocalizationState);
    fn on_localization_changed(&self, localizations: HashMap<String, String>);
    fn on_language_code_changed(&self, language_code: Option<String>);
}

struct State {
    localization_state: LocalizationState,
    localizations: HashMap<String, String>,
    language_code: Option<String>,
}

#[derive(uniffi::Object)]
pub struct LocalizationStateManager {
    me: Weak<LocalizationStateManager>,
    repository: Arc<dyn LocalizationRepository>,
    state: Mutex<State>,
    observer: Mutex<Option<Arc<dyn LocalizationStateObserver>>>,
    observe_task: Mutex<Option<JoinHandle<()>>>,
}

impl LocalizationStateManager {
    pub(crate) fn new(repository: Arc<dyn LocalizationRepository>) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            repository,
            state: Mutex::new(State {
                localization_state: LocalizationState::Loading {},
                localizations: HashMap::new(),
                language_code: None,
            }),
            observer: Mutex::new(None),
            observe_task: Mutex::new(None),
        });
    }

    fn observer(&self) -> Option<Arc<dyn LocalizationStateObserver>> {
        return self.observer.lock().unwrap().clone();
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalizationStateManager {
    pub fn observe(&self, observer: Arc<dyn LocalizationStateObserver>) {
        *self.observer.lock().unwrap() = Some(observer.clone());
        let state = self.state.lock().unwrap();
        observer.on_state_changed(state.localization_state.clone());
        observer.on_language_code_changed(state.language_code.clone());
        if state.localization_state == (LocalizationState::Ready {}) {
            observer.on_localization_changed(state.localizations.clone());
        }
    }

    pub fn release(&self) {
        if let Some(task) = self.observe_task.lock().unwrap().take() {
            task.abort();
        }
        *self.observer.lock().unwrap() = None;
        let mut state = self.state.lock().unwrap();
        state.localizations = HashMap::new();
        state.language_code = None;
        state.localization_state = LocalizationState::Loading {};
    }

    pub async fn setup_localization(&self, language_code: String) {
        self.observe_language();
        self.repository.set_language_code(&language_code);
    }

    pub fn set_language_code(&self, language_code: String) {
        self.repository.set_language_code(&language_code);
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.repository.translate(&key);
    }

    pub fn localization_state(&self) -> LocalizationState {
        return self.state.lock().unwrap().localization_state.clone();
    }
}

impl LocalizationStateManager {
    fn observe_language(&self) {
        let mut receiver = self.repository.language_code();
        let weak = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let language_code = receiver.borrow_and_update().clone();
                let Some(manager) = weak.upgrade() else { break };
                if let Some(language_code) = language_code {
                    manager.refresh(&language_code).await;
                }
                if receiver.changed().await.is_err() {
                    break;
                }
            }
        });
        if let Some(previous) = self.observe_task.lock().unwrap().replace(task) {
            previous.abort();
        }
    }

    async fn refresh(&self, language_code: &str) {
        self.emit_language_code(Some(language_code.to_string()));
        self.set_state(LocalizationState::Loading {});

        match self.repository.load(language_code).await {
            Ok(localization) => self.apply(localization.localizations),
            Err(error) => {
                let state = if error.error_type == ErrorType::Cache {
                    LocalizationState::Loading {}
                } else {
                    LocalizationState::Error {
                        message: error.message,
                    }
                };
                self.set_state(state);
            }
        }
    }

    fn apply(&self, localizations: HashMap<String, String>) {
        {
            let mut state = self.state.lock().unwrap();
            state.localizations = localizations.clone();
            state.localization_state = LocalizationState::Ready {};
        }
        if let Some(observer) = self.observer() {
            observer.on_localization_changed(localizations);
            observer.on_state_changed(LocalizationState::Ready {});
        }
    }

    fn emit_language_code(&self, language_code: Option<String>) {
        self.state.lock().unwrap().language_code = language_code.clone();
        if let Some(observer) = self.observer() {
            observer.on_language_code_changed(language_code);
        }
    }

    fn set_state(&self, state: LocalizationState) {
        self.state.lock().unwrap().localization_state = state.clone();
        if let Some(observer) = self.observer() {
            observer.on_state_changed(state);
        }
    }
}
