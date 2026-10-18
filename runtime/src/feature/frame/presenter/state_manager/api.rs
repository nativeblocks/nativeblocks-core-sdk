use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use crate::feature::frame::domain::model::{NativeActionModel, NativeBlockModel};
use crate::feature::frame::domain::repository::{FrameRepository, FrameResult};
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::feature::frame::presenter::state_manager::model::{
    ActionLogEvent, BlockLogEvent, FrameChangeType, FrameFull,
};
use crate::feature::frame::presenter::state_manager::observer::Observer;
use crate::feature::frame::presenter::state_manager::change_queue::ChangeQueue;
use crate::feature::frame::presenter::state_manager::snapshot::{FrameSnapshot, SnapshotStore};
use crate::feature::frame::presenter::state_manager::refs;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use crate::feature::frame::presenter::state_manager::{action, block, snapshot, variable};
use crate::library::result::ErrorType;
use crate::plugin::global_parameter::GlobalParameterProvider;

#[uniffi::export(with_foreign)]
pub trait FrameStateObserver: Send + Sync {
    fn on_frame_change(&self, change: FrameChangeType);
}

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    me: Weak<FrameStateManager>,
    internal_state: Mutex<InternalState>,
    subscription: Observer,
    pending_changes: ChangeQueue,
    snapshots: SnapshotStore,
    repository: Arc<dyn FrameRepository>,
    globals: Arc<GlobalParameterProvider>,
    logger: FrameLogger,
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
            pending_changes: ChangeQueue::default(),
            snapshots: SnapshotStore::new(snapshots),
            repository,
            globals,
            logger,
        });
    }

    fn clear_frame(&self) {
        let mut current_state = self.internal_state.lock().unwrap();
        *current_state = InternalState::fresh();
    }

    fn observe(
        &self,
        route: String,
        args: HashMap<String, String>,
        globals: Arc<HashMap<String, String>>,
        state_key: Option<String>,
        observer: Arc<dyn FrameStateObserver>,
    ) {
        self.snapshots.open(state_key);
        self.clear_frame();
        self.pending_changes.clear();
        self.subscription.set(observer);

        let mut subscriber = self.repository.subscribe(&route);
        let weak_manager = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let frame_result = subscriber.borrow_and_update().clone();
                let Some(manager) = weak_manager.upgrade() else {
                    break;
                };
                manager.apply_frame(frame_result, &args, &globals);
                if subscriber.changed().await.is_err() {
                    break;
                }
            }
        });

        self.subscription.attach(task);
    }

    fn release_subscription(&self) {
        self.subscription.clear();
        self.pending_changes.clear();
        self.clear_frame();
    }

    fn deliver_pending_changes(&self) {
        self.pending_changes.deliver_pending(|change| self.subscription.emit(change));
    }

    fn change_variable(&self, key: &str, value: String) {
        {
            let mut internal_state = self.internal_state.lock().unwrap();
            if variable::is_host_provided(&internal_state, key) {
                self.logger.host_variable_write(key);
                return;
            }
            let Some(diff) = variable::change(&mut internal_state, key, value) else {
                return;
            };
            self.pending_changes.enqueue(FrameChangeType::Diff { frame: diff });
        }
        self.snapshots.save(&self.internal_state.lock().unwrap());
        self.logger.variable_changed(key);
        self.deliver_pending_changes();
    }

    fn apply_frame(
        &self,
        result: FrameResult,
        args: &HashMap<String, String>,
        globals: &HashMap<String, String>,
    ) {
        let snapshot = self.snapshots.active();
        let rendering_state = {
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
                snapshot::restore(&mut internal_state, snapshot);
            }
            let mut full = frame_full(
                &internal_state,
                block::render(&internal_state, &self.logger),
                action::render(&internal_state, &self.logger),
            );
            full.restored = snapshot.is_some();
            let rendering_state = full.state.clone();
            self.pending_changes.enqueue(FrameChangeType::Full { frame: full });
            rendering_state
        };
        self.deliver_pending_changes();
        self.logger.frame_state_changed(&rendering_state);
    }
}

fn frame_full(
    state: &InternalState,
    blocks: HashMap<String, NativeBlockModel>,
    actions: HashMap<String, Vec<NativeActionModel>>,
) -> FrameFull {
    return FrameFull {
        state: state.state.clone(),
        root_key: state.root_key.clone(),
        blocks,
        variables: refs::resolved_all(&state.variables),
        actions,
        restored: false,
    };
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub async fn setup_frame(
        &self,
        route: String,
        args: HashMap<String, String>,
        state_key: Option<String>,
        observer: Arc<dyn FrameStateObserver>,
    ) {
        self.logger.set_route(&route);
        let globals = Arc::new(self.globals.get());
        self.observe(route.clone(), args, globals.clone(), state_key, observer);
        let _ = self.repository.load(&route, &globals).await;
    }

    pub fn log_action(&self, event: ActionLogEvent) {
        self.logger.action(event);
    }

    pub fn log_block(&self, event: BlockLogEvent) {
        self.logger.block(event);
    }

    pub fn release(&self) {
        self.release_subscription();
    }

    pub fn update_variable(&self, key: String, value: String) {
        self.change_variable(&key, value);
    }
}
