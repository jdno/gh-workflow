/// Generate a string type
///
/// The `name!` macro generates a string type. The type implements common traits and conversations
/// that make it easy to use.
///
/// # Example
///
/// ```rust
/// use gh_workflow::name;
///
/// name!(WorkflowName);
///
/// let workflow_name: WorkflowName = "workflow".into();
/// println!("Workflow {}", workflow_name);
/// ```
#[macro_export]
macro_rules! name {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(String);

        impl $name {
            /// Initializes a new name
            pub fn new(name: &str) -> Self {
                Self(name.into())
            }

            /// Returns the inner value of the name
            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(string: &str) -> $name {
                $name(string.into())
            }
        }

        impl From<String> for $name {
            fn from(string: String) -> $name {
                $name(string)
            }
        }
    };
}
