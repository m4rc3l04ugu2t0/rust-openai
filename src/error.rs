pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

// use derive_more::From;

// #[derive(Debug, From)]
// pub enum Error {
//     // -- Temporary
//     #[from]
//     Custom(String),

//     // -- Stds
//     #[from]
//     Io(std::io::Error),

//     // -- Externals
//     #[from]
//     Glob(globset::Error),

//     #[from]
//     SerdeJson(serde_json::Error),
// }
