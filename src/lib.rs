mod write;

// pub use says that we have a function called write,
// in the module called write, and that the function will
// be exposed publicly, so people that have digital_garden installed
// will be able to use the write function from the write module.
pub use write::write;
