#![no_std]

mod command;
mod harness;
mod port;
mod syscall;

pub use command::{Command, Executor};
pub use harness::Harness;
pub use port::{HarnessPort, MemPort};
pub use syscall::{syscall3, syscall6};

/// Wrap a foreign-defined (typically `km_command`) command as a harness
/// command (command that can be executed on the target kernel) .
///  Implement `Deref`.
///
/// If `execute_fn` is provided, it will be used to implement `Command` trait.
///
/// # Format
///
/// - `harness_command!(mod1::mod2::..., cmd)`,
/// - `harness_command!(mod1::mod2::..., cmd, { execute_fn })`
#[macro_export]
macro_rules! harness_command {
    ($($mod:ident)::*, $cmd:ident) => {
        struct $cmd($($mod)::*::$cmd);

        impl core::ops::Deref for $cmd {
            type Target = $($mod)::*::$cmd;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $cmd {
            /// Commad id.
            pub const ID: usize = $($mod)::*::$cmd::ID;
        }
    };
    ($($mod:ident)::*, $cmd:ident, $execute_fn:block) => {
        harness_command!($($mod)::*,$cmd);

        impl $crate::Command for $cmd {
            fn execute(&self, output: &mut [u8]) -> isize {
                /// `get!()` => `self`; `get!(field)` => `self.field`
                #[allow(unused_macros)]
                macro_rules! get {
                    () => {
                        self
                    };
                    ($field:ident) => {
                        self.$field
                    };
                }
                /// `data!()` => `data`;
                #[allow(unused_macros)]
                macro_rules! output {
                    () => {
                        output
                    };
                }
                $execute_fn
            }
            fn from_bytes(buf: &[u8]) -> Self {
                Self($($mod)::*::$cmd::from_bytes(buf).unwrap().0)
            }
        }
    };
}

/// Define and implement a command executor.
/// This macro requires `km-command` as dependency.
///
/// Format: `executor!(Executor, cmd1, cmd2, ...)`,
#[macro_export]
macro_rules! executor {
    ($ex:ident, $($cmd:ident),*) => {
        struct $ex;

        impl $crate::Executor for $ex {
            fn parse_and_execute(&self, cmd_buf: &[u8], output: &mut [u8]) -> isize {
                let (id, remain) = km_command::id_from_bytes(cmd_buf);
                match id {
                    $($cmd::ID => {
                        let cmd = $cmd::from_bytes(remain);
                        cmd.execute(output)
                    },)*
                    _ => panic!("Unknown command: {}", id),
                }
            }
        }
    };
}

/// Make a syscall with a variable number of arguments.
///
/// Format: `syscall!(id, arg0, arg1, ...)`
#[macro_export]
macro_rules! syscall {
    ($id:expr) => {
        $crate::syscall3($id, [0, 0, 0])
    };
    ($id:expr,$arg0:expr) => {
        $crate::syscall3($id, [$arg0 as usize, 0, 0])
    };
    ($id:expr,$arg0:expr,$arg1:expr) => {
        $crate::syscall3($id, [$arg0 as usize, $arg1 as usize, 0])
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr) => {
        $crate::syscall3($id, [$arg0 as usize, $arg1 as usize, $arg2 as usize])
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr) => {
        $crate::syscall6(
            $id,
            [
                $arg0 as usize,
                $arg1 as usize,
                $arg2 as usize,
                $arg3 as usize,
                0,
                0,
            ],
        )
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr,$arg4:expr) => {
        $crate::syscall6(
            $id,
            [
                $arg0 as usize,
                $arg1 as usize,
                $arg2 as usize,
                $arg3 as usize,
                $arg4 as usize,
                0,
            ],
        )
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr,$arg4:expr,$arg5:expr) => {
        $crate::syscall6(
            $id,
            [
                $arg0 as usize,
                $arg1 as usize,
                $arg2 as usize,
                $arg3 as usize,
                $arg4 as usize,
                $arg5 as usize,
            ],
        )
    };
}
