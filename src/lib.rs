mod uuid32;

#[cfg(feature = "postgres")]
mod uuid32_postgres;

pub use crate::uuid32::Uuid32;
