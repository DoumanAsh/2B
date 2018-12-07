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

pub trait ResultExt<T, E> {
    fn unreach_err(self) -> T;
    fn unreach_ok(self) -> E;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn unreach_err(self) -> T {
        match self {
            Ok(res) => res,
            Err(_) => unreach!()
        }
    }

    fn unreach_ok(self) -> E {
        match self {
            Err(res) => res,
            Ok(_) => unreach!()
        }
    }
}
