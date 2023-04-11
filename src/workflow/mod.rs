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
}

name!(
    /// # The name of a workflow
    ///
    /// GitHub displays the names of your workflows on your repository's "Actions" tab. If you omit
    /// it, GitHub sets it to the workflow file path relative to the root of the repository.
    WorkflowName
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
        };

        assert_eq!(&Some(WorkflowName::new("workflow")), workflow.name());
    }

    #[test]
    fn set_name() {
        let mut workflow = Workflow { name: None };

        workflow.set_name(WorkflowName::new("workflow"));

        assert_eq!(&Some(WorkflowName::new("workflow")), workflow.name());
    }

    #[test]
    fn trait_display_with_name() {
        let workflow = Workflow {
            name: Some("workflow".into()),
        };

        assert_eq!("workflow", workflow.to_string());
    }

    #[test]
    fn trait_display_without_name() {
        let workflow = Workflow { name: None };

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
