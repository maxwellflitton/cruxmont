//! Defines extracting config variables.
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::env;

/// Defines the trait for getting config variables
pub trait GetConfigVariable {
    /// Gets the config variable
    ///
    /// # Arguments
    /// * `variable` - The name of the config variable to get
    ///
    /// # Returns
    /// * `Result<String, String>` - The result of getting the config variable
    fn get_config_variable(variable: String) -> Result<String, NanoServiceError>;
}

/// Defines the struct for getting config variables from the environment
pub struct EnvConfig;

impl GetConfigVariable for EnvConfig {
    /// Gets the config variable from the environment
    ///
    /// # Arguments
    /// * `variable` - The name of the config variable to get
    ///
    /// # Returns
    /// * `Result<String, NanoServiceError>` - The result of getting the config variable
    fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
        match env::var(&variable) {
            Ok(val) => Ok(val),
            Err(_) => Err(NanoServiceError::new(
                format!("{} not found in environment", variable),
                NanoServiceErrorStatus::Unknown,
            )),
        }
    }
}

/// Defines a static config (useful for testing). We can run the by the following:
///
/// ```
/// use kernel::define_static_config;
///
/// define_static_config!(
///     TestConfig,
///     "one" => "1",
///     "two" => "2"
/// );
/// ```
#[macro_export]
macro_rules! define_static_config {
    ($handle:ident, $( $key:expr => $value:expr ),*) => {
        #[derive(Clone)]
        pub struct $handle;
        impl kernel::config::GetConfigVariable for $handle {
            fn get_config_variable(variable: String) -> Result<String, kernel::errors::NanoServiceError> {
                match variable.as_str() {
                    $(
                        $key => Ok($value.to_string()),
                    )*
                    _ => Err(kernel::errors::NanoServiceError::new(
                        format!("key: {} was not found", variable),
                        kernel::errors::NanoServiceErrorStatus::Unknown
                    ))
                }
            }
        }
    };
    // below should be called if using the macro in the kernel crate
    (KERNEL, $handle:ident, $( $key:expr => $value:expr ),*) => {
        pub struct $handle;
        impl $crate::config::GetConfigVariable for $handle {
            fn get_config_variable(variable: String) -> Result<String, $crate::errors::NanoServiceError> {
                match variable.as_str() {
                    $(
                        $key => Ok($value.to_string()),
                    )*
                    _ => Err($crate::errors::NanoServiceError::new(
                        format!("key: {} was not found", variable),
                        $crate::errors::NanoServiceErrorStatus::Unknown
                    ))
                }
            }
        }
    };
}
