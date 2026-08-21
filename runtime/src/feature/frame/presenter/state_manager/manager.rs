use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use crate::feature::frame::domain::repository::{FrameRepository, FrameResult};
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::feature::frame::presenter::state_manager::api::FrameStateObserver;
use crate::feature::frame::presenter::state_manager::model::FrameChangeType;
use crate::feature::frame::presenter::state_manager::observer::Observer;
use crate::feature::frame::presenter::state_manager::state::{FrameSnapshot, InternalState};
use crate::library::result::ErrorType;
use crate::plugin::global_parameter::GlobalParameterProvider;

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    me: Weak<FrameStateManager>,
    internal_state: Mutex<InternalState>,
    subscription: Observer,
    state_key: Mutex<Option<String>>,
    snapshots: Arc<Mutex<HashMap<String, FrameSnapshot>>>,
    pub(super) repository: Arc<dyn FrameRepository>,
    pub(super) globals: Arc<GlobalParameterProvider>,
    pub(super) logger: FrameLogger,
}

impl FrameStateManager {
    pub(crate) fn new(
        repository: Arc<dyn FrameRepository>,
        globals: Arc<GlobalParameterProvider>,
        logger: FrameLogger,
        snapshots: Arc<Mutex<HashMap<String, FrameSnapshot>>>,
    ) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            internal_state: Mutex::new(InternalState::fresh()),
            subscription: Observer::empty(),
            state_key: Mutex::new(None),
            snapshots,
            repository,
            globals,
            logger,
        });
    }

    pub(super) fn observe(
        &self,
        route: String,
        args: HashMap<String, String>,
        globals: Arc<HashMap<String, String>>,
        state_key: Option<String>,
        observer: Arc<dyn FrameStateObserver>,
    ) {
        self.set_state_key(state_key);
        *self.internal_state.lock().unwrap() = InternalState::fresh();

        let mut frame_receiver = self.repository.subscribe(&route);
        let weak_manager = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let frame_result = frame_receiver.borrow_and_update().clone();
                let Some(manager) = weak_manager.upgrade() else {
                    break;
                };
                manager.apply_frame(frame_result, &args, &globals);
                if frame_receiver.changed().await.is_err() {
                    break;
                }
            }
        });

        self.subscription.set(observer, task);
    }

    pub(super) fn release_subscription(&self) {
        self.subscription.clear();
        *self.internal_state.lock().unwrap() = InternalState::fresh();
    }

    pub(super) fn change_variable(&self, key: &str, value: String) {
        let diff = {
            let mut internal_state = self.internal_state.lock().unwrap();
            if internal_state.is_injected_variable(key) {
                self.logger.injected_variable_write(key);
                return;
            }
            internal_state.change_variable(key, value)
        };
        let Some(diff) = diff else {
            return;
        };
        self.save_snapshot();
        self.logger.variable_changed(key, diff.blocks.len());
        self.subscription
            .emit(FrameChangeType::Diff { frame: diff });
    }

    pub(super) fn change_block_data(&self, block_key: &str, data_key: &str, value: String) {
        let key = {
            let internal_state = self.internal_state.lock().unwrap();
            internal_state.variable_key_of(block_key, data_key)
        };
        let Some(key) = key else {
            return;
        };
        self.change_variable(&key, value);
    }

    #[deprecated(note = "Properties are being replaced by data.")]
    #[allow(deprecated)]
    pub(super) fn change_block_property(
        &self,
        block_key: String,
        property_key: String,
        value_mobile: String,
        value_tablet: String,
        value_desktop: String,
    ) {
        let diff = {
            let mut internal_state = self.internal_state.lock().unwrap();
            internal_state.change_block_property(
                block_key,
                property_key,
                value_mobile,
                value_tablet,
                value_desktop,
            )
        };
        let Some(diff) = diff else {
            return;
        };
        self.save_snapshot();
        self.subscription
            .emit(FrameChangeType::Diff { frame: diff });
    }

    fn save_snapshot(&self) {
        let Some(key) = self.state_key.lock().unwrap().clone() else {
            return;
        };
        let snapshot = self.internal_state.lock().unwrap().snapshot();
        self.snapshots.lock().unwrap().insert(key, snapshot);
    }

    fn set_state_key(&self, state_key: Option<String>) {
        let mut current = self.state_key.lock().unwrap();
        if *current == state_key {
            return;
        }
        if let Some(previous) = std::mem::replace(&mut *current, state_key) {
            self.snapshots.lock().unwrap().remove(&previous);
        }
    }

    fn get_active_snapshot(&self) -> Option<FrameSnapshot> {
        let key = self.state_key.lock().unwrap().clone()?;
        return self.snapshots.lock().unwrap().get(&key).cloned();
    }

    fn apply_frame(
        &self,
        result: FrameResult,
        args: &HashMap<String, String>,
        globals: &HashMap<String, String>,
    ) {
        let snapshot = self.get_active_snapshot();
        let mut full = {
            let mut internal_state = self.internal_state.lock().unwrap();
            *internal_state = match result {
                Ok(frame) => InternalState::ready(frame, args, globals),
                Err(error) => {
                    if error.error_type == ErrorType::Cache {
                        InternalState::fresh()
                    } else {
                        InternalState::error(error.message)
                    }
                }
            };
            if let Some(snapshot) = &snapshot {
                internal_state.restore(snapshot);
            }
            internal_state.to_frame_full()
        };
        full.restored = snapshot.is_some();
        let rendering_state = full.state.clone();
        self.subscription
            .emit(FrameChangeType::Full { frame: full });
        self.logger.frame_state_changed(&rendering_state);
    }
}
