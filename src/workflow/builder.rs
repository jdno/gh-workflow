use std::fmt::{Display, Formatter};

/// Builder for Workflows
///
/// Workflows can be constructed using the builder pattern. The builder provides methods to set the
/// fields of a [`Workflow`], and then a [`build`] method to construct the [`Workflow`]. The method
/// returns a [`Result`] that is [`Ok`] if the [`Workflow`] was successfully constructed, and an
/// [`Err`] if mandatory fields for the the [`Workflow`] were missing.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WorkflowBuilder {}

impl WorkflowBuilder {
    /// Create a new WorkflowBuilder
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for WorkflowBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorkflowBuilder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let workflow_builder = WorkflowBuilder {};
        assert_eq!("WorkflowBuilder", workflow_builder.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<WorkflowBuilder>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<WorkflowBuilder>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<WorkflowBuilder>();
    }
}
