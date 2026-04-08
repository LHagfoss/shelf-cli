pub mod add;
pub mod clear;
pub mod copy; // exports the module
pub mod drop;
pub mod paste;
pub mod peak;
pub mod pop;

pub use add::add;
pub use clear::clear;
pub use copy::copy; // exports the function
pub use drop::drop;
pub use paste::paste;
pub use peak::peak;
pub use pop::pop;
