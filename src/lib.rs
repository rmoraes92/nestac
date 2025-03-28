//! # Nestac
//! _(short for (ne)sted (st)ructure (ac)cess)_ is library to access nested
//! structures using path-like string format.
//!
//! If you work with Python you're probably familiar with
//! [glom](https://glom.readthedocs.io/en/latest/) and that is where the ideia
//! came from.
//!
//! The necessity to make an implementation in Rust comes from a project
//! initially developed in Python that had to loop through of .json files to
//! update their properties using
//! [glom](https://glom.readthedocs.io/en/latest/).
//!
//! Once we start increase the workload to 1k+ inputs the Python script started
//! to present performance issues. That lead to the decision of rewrite the
//! application using Rust but it would still need to support the path strings
//! to make the property updates like.
//!
//! So here we are.

pub mod json;
pub mod toml;

pub use json::read as json_read;
pub use json::update as json_update;
pub use toml::get_paths as toml_get_paths;
pub use toml::read as toml_read;
pub use toml::update as toml_update;
