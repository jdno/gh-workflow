use std::fmt::{Display, Formatter};

use crate::name;

pub use self::builder::WorkflowBuilder;

mod builder;

/// # A GitHub Actions Workflow
///
/// A workflow is a configurable automated process that will run one or more jobs. Workflows are
/// defined by a YAML file checked in to your repository and will run when triggered by an event in
/// your repository, or they can be triggered manually, or at a defined schedule.
///
/// Workflows are defined in the `.github/workflows` directory in a repository, and a repository can
/// have multiple workflows, each of which can perform a different set of tasks. For example, you
/// can have one workflow to build and test pull requests, another workflow to deploy your
/// application every time a release is created, and still another workflow that adds a label every
/// time someone opens a new issue.
///
/// -- [GitHub Actions Documentation](https://docs.github.com/en/actions/using-workflows/about-workflows)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Workflow {
    name: Option<WorkflowName>,
    run_name: Option<WorkflowRunName>,
}

name!(
    /// # The name of a workflow
    ///
    /// GitHub displays the names of your workflows on your repository's "Actions" tab. If you omit
    /// it, GitHub sets it to the workflow file path relative to the root of the repository.
    WorkflowName
);

name!(
    /// # The name for workflow runs generated from the workflow
    ///
    /// GitHub displays the workflow run name in the list of workflow runs on your repository's
    /// "Actions" tab. If `run-name` is omitted or is only whitespace, then the run name is set to
    /// event-specific information for the workflow run. For example, for a workflow triggered by a
    /// `push` or `pull_request` event, it is set as the commit message.
    WorkflowRunName
);

impl Workflow {
    /// Returns the name of the workflow
    pub fn name(&self) -> &Option<WorkflowName> {
        &self.name
    }

    /// Sets the name of the workflow
    pub fn set_name(&mut self, name: WorkflowName) {
        self.name = Some(name);
    }

    /// Returns the run-name of the workflow
    pub fn run_name(&self) -> &Option<WorkflowRunName> {
        &self.run_name
    }

    /// Sets the run-name of the workflow
    pub fn set_run_name(&mut self, run_name: WorkflowRunName) {
        self.run_name = Some(run_name);
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            return write!(f, "{}", name);
        }

        write!(f, "<unnamed workflow>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let workflow = Workflow {
            name: Some("workflow".into()),
            ..Default::default()
        };

        assert_eq!(&Some(WorkflowName::new("workflow")), workflow.name());
    }

    #[test]
    fn set_name() {
        let mut workflow = Workflow {
            name: None,
            ..Default::default()
        };

        workflow.set_name(WorkflowName::new("workflow"));

        assert_eq!(&Some(WorkflowName::new("workflow")), workflow.name());
    }

    #[test]
    fn run_name() {
        let workflow = Workflow {
            run_name: Some("workflow".into()),
            ..Default::default()
        };

        assert_eq!(&Some(WorkflowRunName::new("workflow")), workflow.run_name());
    }

    #[test]
    fn set_run_name() {
        let mut workflow = Workflow {
            run_name: None,
            ..Default::default()
        };

        workflow.set_run_name(WorkflowRunName::new("workflow"));

        assert_eq!(&Some(WorkflowRunName::new("workflow")), workflow.run_name());
    }

    #[test]
    fn trait_display_with_name() {
        let workflow = Workflow {
            name: Some("workflow".into()),
            ..Default::default()
        };

        assert_eq!("workflow", workflow.to_string());
    }

    #[test]
    fn trait_display_without_name() {
        let workflow = Workflow {
            name: None,
            ..Default::default()
        };

        assert_eq!("<unnamed workflow>", workflow.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Workflow>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Workflow>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Workflow>();
    }
}
