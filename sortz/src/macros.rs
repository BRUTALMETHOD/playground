// to add #[cfg(feature = "stdout")] to enable println
macro_rules! debugprintln {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}

pub(crate) use debugprintln;
