use std::fmt::{Display, Formatter};

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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Workflow {}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Workflow")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_display() {
        let workflow = Workflow {};
        assert_eq!("Workflow", workflow.to_string());
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
