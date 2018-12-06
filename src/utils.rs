#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! unreach {
    () => ({
        unsafe {
            core::hint::unreachable_unchecked();
        }
    })
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! unreach {
    () => ({
        unreachable!()
    })
}
