use esp32c3_hal::{interrupt, Rtc};
use esp32c3_hal::clock::ClockControl;
use esp32c3_hal::timer::{TimerGroup, Timer, Timer0};
use esp32c3_hal::rng::Rng;
use esp32c3_hal::sha::{Sha, ShaMode};
use esp32c3_hal::peripherals::{Interrupt, TIMG0};
use esp32c3_hal::systimer::SystemTimer;
use esp32c3_hal::prelude::*; //All these stupid traits are inside prelude
use esp32c3_hal::esp_riscv_rt::riscv;

use ufmt_stdio::ufmt;

use core::{marker, ops, ptr, mem};
use core::cell::UnsafeCell;

const SHA_ALGO: ShaMode = ShaMode::SHA1;

struct Instance(UnsafeCell<mem::MaybeUninit<Peripherals>>);

unsafe impl Sync for Instance {}

static INSTANCE: Instance = Instance(UnsafeCell::new(mem::MaybeUninit::uninit()));

///Returns number of ticks since boot
pub fn ticks() -> u64 {
    SystemTimer::now()
}

///Timestamp with seconds and remaining part separated.
///
///Formatted as `<secs>.<rem>`
pub struct Timestamp {
    pub secs: u64,
    pub rem: u64,
}

impl Timestamp {
    #[inline(always)]
    pub fn uptime() -> Self {
        let ticks = ticks();
        Self {
            secs: match ticks.checked_div(SystemTimer::TICKS_PER_SECOND) {
                Some(secs) => secs,
                None => unreach!(),
            },
            rem: match ticks.checked_rem(SystemTimer::TICKS_PER_SECOND) {
                Some(secs) => secs,
                None => unreach!(),
            },
        }
    }
}

impl ufmt::uDisplay for Timestamp {
    #[inline(never)]
    fn fmt<W: ufmt::uWrite + ?Sized>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error> {
        let _ = ufmt::uDisplay::fmt(&self.secs, fmt);
        let _ = fmt.write_str(".");
        ufmt::uDisplay::fmt(&self.rem, fmt)
    }
}

///Initializes peripherals and enables interrupts.
///
///Note that this function must be called exactly once as the first thing in your main
pub fn init() {
    unsafe {
        riscv::interrupt::disable();
        *INSTANCE.0.get() = mem::MaybeUninit::new(Peripherals::new());
        riscv::interrupt::enable();
    }
}

#[inline(always)]
///Returns Peripherals instance
///
///This function require `init` to be called prior.
pub fn instance() -> &'static Peripherals {
    unsafe {
        &*(*(INSTANCE.0.get())).as_ptr()
    }
}

#[repr(transparent)]
///This construct can be used to work around `noalias` limitation of mutable references.
///As we store `&mut T` as `*mut T` we're no longer triggering UB by having multiple `&mut T`.
pub struct Ref<'a, T> {
    inner: ptr::NonNull<T>,
    lifetime: marker::PhantomData<&'a T>,
}

impl<'a, T> Ref<'a, T> {
    #[inline(always)]
    fn new(ptr: *mut T) -> Self {
        Self {
            inner: match ptr::NonNull::new(ptr) {
                Some(inner) => inner,
                None => unsafe {
                    core::hint::unreachable_unchecked()
                }
            },
            lifetime: marker::PhantomData,
        }
    }
}

impl<'a, T> ops::Deref for Ref<'a, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.inner.as_ref()
        }
    }
}

impl<'a, T> ops::DerefMut for Ref<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.inner.as_mut()
        }
    }
}

#[repr(transparent)]
///Wrapper type to provide unique mutable accesses in convenient way without baby mode with RefCell
pub struct Accessor<T>(UnsafeCell<T>);

impl<T> Accessor<T> {
    #[inline(always)]
    ///Invokes callback within critical section, passing inner self as mutable
    ///
    ///This can be called recrusively, but creating multiple mutable references when you use them.
    ///Even MIRI cannot figure out whether multiple aliasing borrows are safe after all.
    ///But technically it is very unlikely to be an issue as long as there is it is within critical
    ///section
    pub fn with<R, F: FnOnce(Ref<'_, T>) -> R>(&self, cb: F) -> R {
        let ptr = self.0.get();
        critical_section::with(|_| {
            cb(Ref::new(ptr))
        })
    }
}

impl<T> ops::Deref for Accessor<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0.get()
        }
    }
}

impl<T> From<T> for Accessor<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }
}

///Used peripherals
///
///Reserve for future:
///- SYSTIMER - to be used by radio stack
pub struct Peripherals {
    pub rtc: Accessor<Rtc<'static>>,
    pub rng: Accessor<Rng<'static>>,
    pub sha: Accessor<Sha<'static>>,
    pub timer0: Accessor<Timer<Timer0<TIMG0>>>,
}

impl Peripherals {
    fn new() -> Self {
        let peripherals = unsafe {
            esp32c3_hal::peripherals::Peripherals::steal()
        };

        let mut system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

        let mut rtc = Rtc::new(peripherals.RTC_CNTL);

        let mut timer_group0 = TimerGroup::new(
            peripherals.TIMG0,
            &clocks,
            &mut system.peripheral_clock_control,
        );
        let mut timer_group1 = TimerGroup::new(
            peripherals.TIMG1,
            &clocks,
            &mut system.peripheral_clock_control,
        );

        // Disable watchdog timers
        rtc.swd.disable();
        rtc.rwdt.disable();
        timer_group0.wdt.disable();
        timer_group1.wdt.disable();

        let rng = Rng::new(peripherals.RNG).into();
        let sha = Sha::new(peripherals.SHA, SHA_ALGO, &mut system.peripheral_clock_control).into();

        timer_group0.timer0.start(1u32.secs());
        timer_group0.timer0.listen();
        let _ = interrupt::enable(Interrupt::TG0_T0_LEVEL, interrupt::Priority::Priority1);

        Self {
            rtc: rtc.into(),
            rng,
            sha,
            timer0: timer_group0.timer0.into(),
        }
    }
}

unsafe impl Sync for Peripherals {}
