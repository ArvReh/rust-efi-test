// Re-export the target architectues module as 'arch'
#[cfg(target_arch="x86_64")]
pub use self::x86_64 as arch;

pub mod x86_64;
