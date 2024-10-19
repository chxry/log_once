pub use log;

#[macro_export]
macro_rules! log_once {
  ($lvl:expr, $($arg:tt)+) => (
    unsafe {
      static mut _LOG_FLAG: bool = false;
      if !_LOG_FLAG {
        $crate::log::log!($lvl, $($arg)+);
        _LOG_FLAG = true;
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
  env_logger::builder()
    .is_test(true)
    .filter_level(log::LevelFilter::Info)
    .init();
  info_once!("real");
  for i in 0..3 {
    info_once!("test {}", i);
  }
}
