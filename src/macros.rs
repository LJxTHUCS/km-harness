//! Macros used to construct harness.

/// Wrap a command as a harness command. Implement `Deref` and `serde::Deserialize`.
///
/// Format: `harness_command!(mod1::mod2::..., cmd)`
/// 
/// where `mod1::mod2::...::cmd` is the original definition of the command.
#[macro_export]
macro_rules! harness_command {
    ($($mod:ident)::*,$cmd:ident) => {
        struct $cmd($($mod)::*::$cmd);
        
        impl<'de> serde::Deserialize<'de> for $cmd {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let cmd = <$($mod::)*$cmd>::deserialize(deserializer)?;
                Ok(Self(cmd))
            }
        }

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
}

/// Define and implement a command executor.
/// 
/// Format: `executor!(Executor, deserializer, cmd1, cmd2, ...)`
/// 
/// where `Executor` is the name of the executor struct, `deserializer` is the 
/// deserializing function.
#[macro_export]
macro_rules! executor {
    (
        $ex:ident,
        $de:ident,
        $($cmd:ident),*
    ) => {
        struct $ex;

        impl harness::Executor for $ex {
            fn deser_and_exec(
                &self,
                buf: &[u8]
            ) -> core::result::Result<isize, ()> {
                use core::convert::TryInto;
                let cmd_id = $de::<u64>(buf).unwrap() as usize;
                match cmd_id {
                    $($cmd::ID => {
                        let command = $de::<$cmd>(buf).map_err(|_| ())?;
                        let ret = command.execute();
                        core::result::Result::Ok(ret)
                    })*
                    _ => core::result::Result::Err(()),
                }
            }
        }
    };
}

/// Make a syscall with a variable number of arguments.
#[macro_export]
macro_rules! syscall {
    ($id:expr) => { harness::syscall3($id, [0, 0, 0]) };
    ($id:expr,$arg0:expr) => { harness::syscall3($id, [$arg0 as usize, 0, 0]) };
    ($id:expr,$arg0:expr,$arg1:expr) => { 
        harness::syscall3($id, [$arg0 as usize, $arg1 as usize, 0]) 
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr) => { 
        harness::syscall3($id, [$arg0 as usize, $arg1 as usize, $arg2 as usize]) 
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr) => { 
        harness::syscall6($id, [$arg0 as usize, $arg1 as usize, $arg2 as usize, $arg3 as usize, 0, 0])
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr,$arg4:expr) => { 
        harness::syscall6($id, [$arg0 as usize, $arg1 as usize, 
            $arg2 as usize, $arg3 as usize, $arg4 as usize, 0]) 
    };
    ($id:expr,$arg0:expr,$arg1:expr,$arg2:expr,$arg3:expr,$arg4:expr,$arg5:expr) => { 
        harness::syscall6($id, [$arg0 as usize, $arg1 as usize, $arg2 as usize, 
            $arg3 as usize, $arg4 as usize, $arg5 as usize]) 
    };
}
