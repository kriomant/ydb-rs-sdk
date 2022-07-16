use std::collections::HashMap;

pub(crate) struct OperationParams {
    operation_mode: OperationMode,
    operation_timeout: crate::grpc_wrapper::raw_common_types::Duration,
    cancel_after: crate::grpc_wrapper::raw_common_types::Duration,
    labels: HashMap<String, String>,
}

impl OperationParams {
    fn new_with_timeouts(
        operation_timeout: std::time::Duration,
        cancel_after: std::time::Duration,
    ) -> Self {
        return Self {
            operation_mode: OperationMode::Sync,
            operation_timeout: operation_timeout.into(),
            cancel_after: cancel_after.into(),
            labels: Default::default(),
        };
    }
}

impl From<OperationParams> for ydb_grpc::ydb_proto::operations::OperationParams {
    fn from(params: OperationParams) -> Self {
        Self {
            operation_mode: params.operation_mode.into(),
            operation_timeout: None,
            cancel_after: None,
            labels: Default::default(),
            ..Self::default()
        }
    }
}

pub(crate) enum OperationMode {
    Unspecified,
    Sync,
    Async,
}

use ydb_grpc::ydb_proto::operations::operation_params::OperationMode as GrpcOperationMode;
impl From<OperationMode> for i32 {
    fn from(mode: OperationMode) -> Self {
        let val = match mode {
            OperationMode::Unspecified => GrpcOperationMode::Unspecified,
            OperationMode::Sync => GrpcOperationMode::Sync,
            OperationMode::Async => GrpcOperationMode::Async,
        };
        return val as i32;
    }
}

pub(crate) struct Operation {}
