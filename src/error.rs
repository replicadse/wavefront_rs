macro_rules! make_error {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        /// An error type.
        pub struct $name {
            details: String,
        }

        impl $name {
            /// Error type constructor.
            pub fn new(details: &str) -> Self {
                Self {
                    details: details.to_owned(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.details)
            }
        }

        impl std::error::Error for $name {}
    };
}

make_error!(ParserError);
make_error!(WriterError);
