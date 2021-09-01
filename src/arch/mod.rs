// Re-export the target architectues module as 'arch'
#[cfg(target_arch="x86_64")]
pub use self::x86_64::*;
#[cfg(target_arch="x86_64")]
pub mod x86_64;

#[cfg(target_arch="aarch64")]
compile_error!("aarch64 support is work in progress, it won't compile");
#[cfg(target_arch="aarch64")]
pub use self::aarch64::*;
#[cfg(target_arch="aarch64")]
pub mod aarch64;
