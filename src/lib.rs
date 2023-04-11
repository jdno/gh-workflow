//! # Rust Struct for GitHub Actions Workflows
//!
//! [GitHub Actions](https://docs.github.com/en/actions) is a continuous integration and continuous
//! delivery (CI/CD) platform. Users can create workflows that build, test, and deploy code:
//!
//! > A workflow is a configurable automated process that will run one or more jobs. Workflows are
//! > defined by a YAML file checked in to your repository and will run when triggered by an event
//! > in your repository, or they can be triggered manually, or at a defined schedule.
//! > -- [GitHub Actions Documentation](https://docs.github.com/en/actions/using-workflows/about-workflows)
//!
//! This crate implements a representation of the [workflow syntax] that can be used to read and
//! write the workflow files.
//!
//! [workflow syntax]: https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions

// All public items in the crate are documented using the following guidelines:
// https://rust-lang.github.io/api-guidelines/documentation.html
#![deny(missing_docs)]

pub use self::workflow::*;

mod macros;
mod workflow;
