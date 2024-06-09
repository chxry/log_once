#![feature(const_collections_with_hasher)]
use std::collections::HashSet;
use std::any::TypeId;

pub use log;
pub static mut H: HashSet<[u8; ::std::mem::size_of::<TypeId>()]> =
  HashSet::with_hasher(unsafe { std::mem::zeroed() });

#[macro_export]
macro_rules! log_once {
  ($lvl:expr, $($arg:tt)+) => (
    unsafe {
      struct T;
      let id = ::std::mem::transmute::<_, [u8; ::std::mem::size_of::<TypeId>()]>(::std::any::TypeId::of::<T>());
      if !$crate::H.contains(&id) {
        $crate::log::log!($lvl,$($arg)+);
        $crate::H.insert(id);
      }
    }
  )
}

#[macro_export]
macro_rules! error_once {
  ($($arg:tt)+) => ($crate::log_once!($crate::log::Level::Error, $($arg)+))
}

#[macro_export]
macro_rules! warn_once {
  ($($arg:tt)+) => ($crate::log_once!($crate::log::Level::Warn, $($arg)+))
}

#[macro_export]
macro_rules! info_once {
  ($($arg:tt)+) => ($crate::log_once!($crate::log::Level::Info, $($arg)+))
}

#[macro_export]
macro_rules! debug_once {
  ($($arg:tt)+) => ($crate::log_once!($crate::log::Level::Debug, $($arg)+))
}

#[macro_export]
macro_rules! trace_once {
  ($($arg:tt)+) => ($crate::log_once!($crate::log::Level::Trace, $($arg)+))
}

#[test]
fn it_works() {
  ezlogger::init(log::LevelFilter::Info).unwrap();
  info_once!("real");
  for i in 0..3 {
    info_once!("test {}", i);
  }
}
