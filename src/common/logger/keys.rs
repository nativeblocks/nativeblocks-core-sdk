/// Event tags — what happened (ports `LoggerEventTag`).
pub mod tag {
    pub const FRAME_STATE: &str = "FRAME_STATE";
    pub const FRAME_SYNC_STATE: &str = "FRAME_SYNC_STATE";
    pub const HANDLE_ACTION: &str = "HANDLE_ACTION";
    pub const VARIABLE_CHANGE: &str = "VARIABLE_CHANGE";
    pub const BLOCK_CHANGE: &str = "BLOCK_CHANGE";
    pub const LOCALIZATION_STATE: &str = "LOCALIZATION_STATE";
    pub const LOCALIZATION_SYNC_STATE: &str = "LOCALIZATION_SYNC_STATE";
    pub const GLOBAL_PARAMETERS_CHANGE: &str = "GLOBAL_PARAMETERS_CHANGE";
    pub const LANDING_STATE: &str = "LANDING_STATE";
    pub const EXPERIMENT_STATE: &str = "EXPERIMENT_STATE";
    pub const SCAFFOLD_FETCH: &str = "SCAFFOLD_FETCH";
    pub const FALLBACK_ACTION: &str = "FALLBACK_ACTION";
    pub const FALLBACK_BLOCK: &str = "FALLBACK_BLOCK";
    pub const FRAME_CLEAR: &str = "FRAME_CLEAR";
}

/// Event states — the outcome (ports `LoggerEventState`).
pub mod state {
    pub const FRAME_LOADING: &str = "FRAME_LOADING";
    pub const FRAME_LOAD_FAILED: &str = "FRAME_LOAD_FAILED";
    pub const FRAME_LOAD_SUCCEED: &str = "FRAME_LOAD_SUCCEED";
    pub const FRAME_SYNC_FAILED: &str = "FRAME_SYNC_FAILED";
    pub const FRAME_SYNC_SUCCEED: &str = "FRAME_SYNC_SUCCEED";
    pub const LOCALIZATION_LOAD_FAILED: &str = "LOCALIZATION_LOAD_FAILED";
    pub const LOCALIZATION_LOAD_SUCCEED: &str = "LOCALIZATION_LOAD_SUCCEED";
    pub const LOCALIZATION_SYNC_FAILED: &str = "LOCALIZATION_SYNC_FAILED";
    pub const LOCALIZATION_SYNC_SUCCEED: &str = "LOCALIZATION_SYNC_SUCCEED";
    pub const LOCALIZATION_SET: &str = "LOCALIZATION_SET";
    pub const FALLBACK_TRIGGER: &str = "FALLBACK_TRIGGER";
    pub const FALLBACK_BLOCK: &str = "FALLBACK_BLOCK";
    pub const SCAFFOLD_FETCH_FAILED: &str = "SCAFFOLD_FETCH_FAILED";
    pub const SCAFFOLD_FETCH_SUCCEED: &str = "SCAFFOLD_FETCH_SUCCEED";
    pub const EXPERIMENT_FETCH_FAILED: &str = "EXPERIMENT_FETCH_FAILED";
    pub const EXPERIMENT_FETCH_SUCCEED: &str = "EXPERIMENT_FETCH_SUCCEED";
    pub const VARIABLE_UPDATED: &str = "VARIABLE_UPDATED";
    pub const BLOCK_UPDATED: &str = "BLOCK_UPDATED";
    pub const ACTION_EVENT_TRIGGERED: &str = "ACTION_EVENT_TRIGGERED";
    pub const ACTION_EVENT_IGNORED: &str = "ACTION_EVENT_IGNORED";
    pub const TRIGGER_EXECUTED: &str = "TRIGGER_EXECUTED";
}

/// Parameter keys — keys of the structured `parameters` map (ports `LoggerEventParameterKey`).
pub mod parameter {
    pub const STATE: &str = "state";
    pub const EVENT_NAME: &str = "event_name";
    pub const ACTION_NAME: &str = "action_name";
    pub const FRAME_ROUTE: &str = "frame_route";
    pub const ERROR_MESSAGE: &str = "error_message";
    pub const ERROR_TYPE: &str = "error_type";
    pub const ERROR_TAG: &str = "error_tag";
    pub const LANGUAGE_CODE: &str = "language_code";
    pub const KEY_TYPE: &str = "key_type";
    pub const TRIGGER_NAME: &str = "trigger_name";
    pub const THEN: &str = "then";
    pub const KEY: &str = "key";
    pub const FRAMES_COUNT: &str = "frames_count";
    pub const EXPERIMENT_KEY: &str = "experiment_key";
    pub const EXPERIMENT_VALUE: &str = "experiment_value";
    pub const EXPERIMENT_TYPE: &str = "experiment_type";
    pub const PREVIOUS_VALUE: &str = "previous_value";
    pub const NEW_VALUE: &str = "new_value";
    pub const VARIABLE_TYPE: &str = "variable_type";
    pub const BLOCK_KEY: &str = "block_key";
}
