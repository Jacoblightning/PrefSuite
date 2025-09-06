//

/// Macro for running a command and getting the output.
/// To be used inside a function returning Result<?, String>
#[macro_export]
macro_rules! command_output {
    ( $n:expr, $( $x:expr ),* ) => {
        {
            let mut comm = std::process::Command::new($n);
            $(
            comm.arg($x);
            )*
            match comm.output() {
                Ok(o) => {
                    match String::from_utf8(o.stdout) {
                        Ok(o) => o,
                        Err(e) => return Err(e.to_string())
                    }
                }
                Err(e) => return Err(e.to_string())
            }
        }
    };
}

/// Macro for spawning a command
/// To be used inside a function returning Result<?, String>
#[macro_export]
macro_rules! run_command {
    ( $n:expr, $( $x:expr ),* ) => {
        {
            let mut comm = std::process::Command::new($n);
            $(
            comm.arg($x);
            )*
            match comm.spawn() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
        }
    };
}
