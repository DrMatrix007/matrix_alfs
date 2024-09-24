#[macro_export]
macro_rules! command {
    ($t:tt $($args:tt)*) => {
        {
            use std::process::Command;
            Command::new(stringify!($t)) $(.arg(stringify!($args)))*
        }
    };
}


