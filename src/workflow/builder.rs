use std::fmt::{Display, Formatter};

use crate::{WorkflowName, WorkflowRunName};

/// Builder for Workflows
///
/// Workflows can be constructed using the builder pattern. The builder provides methods to set the
/// fields of a [`Workflow`], and then a [`build`] method to construct the [`Workflow`]. The method
/// returns a [`Result`] that is [`Ok`] if the [`Workflow`] was successfully constructed, and an
/// [`Err`] if mandatory fields for the the [`Workflow`] were missing.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WorkflowBuilder {
    name: Option<WorkflowName>,
    run_name: Option<WorkflowRunName>,
}

impl WorkflowBuilder {
    /// Creates a new WorkflowBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name of the workflow
    pub fn name(mut self, name: WorkflowName) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the run-name of the workflow
    pub fn run_name(mut self, run_name: WorkflowRunName) -> Self {
        self.run_name = Some(run_name);
        self
    }
}

impl Display for WorkflowBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "WorkflowBuilder for {}", name),
            None => write!(f, "WorkflowBuilder"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let mut builder = WorkflowBuilder::new();

        builder = builder.name(WorkflowName::new("workflow"));

        assert_eq!(Some(WorkflowName::new("workflow")), builder.name);
    }

    #[test]
    fn run_name() {
        let mut builder = WorkflowBuilder::new();

        builder = builder.run_name(WorkflowRunName::new("workflow"));

        assert_eq!(Some(WorkflowRunName::new("workflow")), builder.run_name);
    }

    #[test]
    fn trait_display_with_name() {
        let workflow_builder = WorkflowBuilder {
            name: Some("workflow".into()),
            ..Default::default()
        };

        assert_eq!("WorkflowBuilder for workflow", workflow_builder.to_string());
    }

    #[test]
    fn trait_display_without_name() {
        let workflow_builder = WorkflowBuilder::new();

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
