use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "runtime-tokio-multi-thread")] {
        mod tokio;
        pub(crate) use self::tokio::*;
        pub(crate) use self::tokio::rt::MultiThreadBuilder as RuntimeBuilder;

    } else if #[cfg(feature = "runtime-tokio-current-thread")] {
        mod tokio;
        pub(crate) use self::tokio::*;
        pub(crate) use self::tokio::rt::CurrentThreadBuilder as RuntimeBuilder;

    } else if #[cfg(feature = "runtime-async-std")] {
        mod async_std;
        pub(crate) use self::async_std::*;
        pub(crate) use self::async_std::rt::Builder as RuntimeBuilder;

    } else {
        mod no_runtime;
        pub(crate) use self::no_runtime::*;
        pub(crate) use self::no_runtime::rt::Builder as RuntimeBuilder;
    }

}
