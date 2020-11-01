#![doc = "Peripheral access API for FE310 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
extern crate vcell;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;

#[allow(unused)]
mod common;

pub use common::{
    aonclk, backup, clint, gpio0, otp, plic, pmu, prci, pwm0, qspi0, rtc, uart0, wdog, AONCLK,
    BACKUP, CLINT, GPIO0, OTP, PLIC, PMU, PRCI, PWM0, PWM1, PWM2, QSPI0, QSPI1, QSPI2, RTC, UART0,
    UART1, WDOG,
};
#[cfg(feature = "g002")]
pub use common::{i2c0, I2C0};

/// All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CLINT"]
    pub CLINT: CLINT,
    #[doc = "PLIC"]
    pub PLIC: PLIC,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "AONCLK"]
    pub AONCLK: AONCLK,
    #[doc = "BACKUP"]
    pub BACKUP: BACKUP,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "PRCI"]
    pub PRCI: PRCI,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "QSPI0"]
    pub QSPI0: QSPI0,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "QSPI1"]
    pub QSPI1: QSPI1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "QSPI2"]
    pub QSPI2: QSPI2,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[cfg(feature = "g002")]
    #[doc = "I2C0"]
    pub I2C0: I2C0,
}

impl Peripherals {
    #[inline]
    fn from_common(peripherals: common::Peripherals) -> Self {
        Self {
            CLINT: peripherals.CLINT,
            PLIC: peripherals.PLIC,
            WDOG: peripherals.WDOG,
            RTC: peripherals.RTC,
            AONCLK: peripherals.AONCLK,
            BACKUP: peripherals.BACKUP,
            PMU: peripherals.PMU,
            PRCI: peripherals.PRCI,
            OTP: peripherals.OTP,
            GPIO0: peripherals.GPIO0,
            UART0: peripherals.UART0,
            QSPI0: peripherals.QSPI0,
            PWM0: peripherals.PWM0,
            UART1: peripherals.UART1,
            QSPI1: peripherals.QSPI1,
            PWM1: peripherals.PWM1,
            QSPI2: peripherals.QSPI2,
            PWM2: peripherals.PWM2,
            #[cfg(feature = "g002")]
            I2C0: peripherals.I2C0,
        }
    }

    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        common::Peripherals::take().map(Self::from_common)
    }

    /// Unchecked version of `Peripherals::take`
    pub unsafe fn steal() -> Self {
        Self::from_common(common::Peripherals::steal())
    }
}
