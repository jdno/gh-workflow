use gh_workflow::{Workflow, WorkflowBuilder};

#[test]
fn builder() {
    let _builder = WorkflowBuilder::new()
        .name("workflow".into())
        .run_name("triggered by test".into());
}

#[test]
fn workflow() {
    let _workflow = Workflow::default();
}
