//! ## Note 1
//!
//! QSPI registers are ordered differently on pads, io & SIO:
//! - PADS: sclk, sd0, sd1, sd2, sd3, ss
//! - IO bank: sclk, ss, sd0, sd1, sd2, sd3
//! - SIO: sclk, ss, sd0, sd1, sd2, sd3
//!
//! This HAL will use the order shared by IO bank & SIO. The main reason for that being the bit
//! shift operation used in SIO and interrupt related registers.
//!
//! ## Note 2
//!
//! The SWD and SWCLK pin only appear on the pad control and cannot be used as gpio.
//! They are therefore absent from this implementation.
//!
//! ## Note 3
//!
//! Dues to limitations in svd2rust and svdtools (and their shared dependencies) it is not possible
//! to fully express the relations between the gpio registers in the different banks on the rp235x
//! at the PAC level.
//!
//! These limitations are respectively:
//! - Inability to derive register with different reset values
//! - Inability to derive from path including clusters and/or arrays
//!
//! This modules bridges that gap by adding a trait definition per register type and implementing it
//! for each of the relevant registers.

use crate::typelevel::Sealed;

use super::{DynFunction, DynPullType};

pub(crate) mod pin_sealed;

/// Value-level `enum` for the pin's bank.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DynBankId {
    /// GPIO Pins bank
    Bank0,
    /// QSPI Pins bank
    Qspi,
}

/// Type-level `enum` for the pin's bank ID.
pub trait BankId: Sealed {}

/// Type-level `variant` of `BankId`
pub struct BankBank0;
impl Sealed for BankBank0 {}
impl BankId for BankBank0 {}

/// Type-level `variant` of `BankId`
pub struct BankQspi;
impl Sealed for BankQspi {}
impl BankId for BankQspi {}

/// Type-level `enum` for the pin Id (pin number + bank).
pub trait PinId: pin_sealed::PinIdOps {
    /// This pin as a `DynPinId`.
    fn as_dyn(&self) -> DynPinId;
}

/// Value-level representation for the pin (bank + id).
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DynPinId {
    /// Pin bank.
    pub bank: DynBankId,
    /// Pin number.
    pub num: u8,
}
impl PinId for DynPinId {
    #[inline]
    fn as_dyn(&self) -> DynPinId {
        *self
    }
}

macro_rules! pin_ids {
    ($bank:ident: $($id:expr;$name:ident),*) => {
        pin_ids!($bank as $bank: $($id;$name),*);
    };
    ($bank:ident as $prefix:ident: $($id:tt),*) => {
        pin_ids!($bank as $prefix: $($id;$id),*);
    };
    ($bank:ident as $prefix:ident: $($id:expr;$name:tt),*) => {
        paste::paste!{
            $(
                #[doc = "Type level variant for the pin `" $name "` in bank `" $prefix "`."]
                pub struct [<$prefix $name>] (pub(crate) ());
                impl crate::typelevel::Sealed for [<$prefix $name>] {}
                impl PinId for [<$prefix $name>] {
                    #[inline]
                    fn as_dyn(&self) -> DynPinId {
                        DynPinId {
                            bank: DynBankId::$bank,
                            num: $id
                        }
                    }
                }
                impl pin_sealed::TypeLevelPinId for [<$prefix $name>] {
                    type Bank = [<Bank $bank>];

                    const ID: DynPinId = DynPinId {
                        bank: DynBankId::$bank,
                        num: $id
                    };

                    fn new() -> Self {
                        Self(())
                    }
                }
            )*
        }
    };
}

/// The bank with all the GPIOs
///
/// GPIOs 0 through 29 are available in all package variants. GPIOs 30 through
/// 47 are available only on the QFN-80 (RP2350B) package.
pub mod bank0 {
    use super::{pin_sealed, BankBank0, DynBankId, DynPinId, PinId};
    pin_ids!(
        Bank0 as Gpio:
         0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,28,29,
        30,31,32,33,34,35,36,37,38,39,
        40,41,42,43,44,45,46,47
    );
}

/// The bank with the QSPI related pins.
///
/// These should all have IDs over 32, because they are all controlled by the
/// hi registers.
pub mod qspi {
    use super::{pin_sealed, BankQspi, DynBankId, DynPinId, PinId};
    pin_ids!(Qspi: 56;UsbDp, 57;UsbDm, 58;Sclk, 59;Ss, 60;Sd0, 61;Sd1, 62;Sd2, 63;Sd3);
}

pub(crate) fn set_function<P: PinId>(pin: &P, function: DynFunction) {
    use crate::pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
    let funcsel = match function {
        // The XIP function has a value of 0, which on bank0 is called JTAG.
        DynFunction::Xip => FUNCSEL_A::JTAG,
        DynFunction::Spi => FUNCSEL_A::SPI,
        DynFunction::Uart => FUNCSEL_A::UART,
        DynFunction::I2c => FUNCSEL_A::I2C,
        DynFunction::Pwm => FUNCSEL_A::PWM,
        DynFunction::Sio(sio) => {
            let mask = pin.mask();
            match sio {
                crate::gpio::DynSioConfig::Input => {
                    pin.sio_oe_clr().write(|w| unsafe { w.bits(mask) });
                }
                crate::gpio::DynSioConfig::Output => {
                    pin.sio_oe_set().write(|w| unsafe { w.bits(mask) });
                }
            }

            FUNCSEL_A::SIO
        }
        DynFunction::Pio0 => FUNCSEL_A::PIO0,
        DynFunction::Pio1 => FUNCSEL_A::PIO1,
        DynFunction::Pio2 => FUNCSEL_A::PIO2,
        DynFunction::Clock => FUNCSEL_A::GPCK,
        DynFunction::Usb => FUNCSEL_A::USB,
        DynFunction::UartAux => FUNCSEL_A::UART_AUX,
        DynFunction::Null => FUNCSEL_A::NULL,
    };

    if funcsel != FUNCSEL_A::NULL {
        pin.pad_ctrl().modify(|_, w| {
            // Set input enable on, output disable off
            // RP2350: input enable defaults to off, so this is important!
            w.ie().set_bit();
            w.od().clear_bit();
            // RP2350: remove pad isolation now a function is wired up
            w.iso().clear_bit();
            w
        });
    } else {
        pin.pad_ctrl().modify(|_, w| {
            // Set input enable off, output disable on
            w.ie().clear_bit();
            w.od().set_bit();
            // RP2350: isolate pad
            w.iso().set_bit();
            w
        });
    }

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    unsafe {
        pin.io_ctrl()
            .write_with_zero(|w| w.funcsel().variant(funcsel));
    }
}

pub(crate) fn set_pull_type<P: PinId>(pin: &P, pull_type: DynPullType) {
    let (pue, pde) = match pull_type {
        DynPullType::None => (false, false),
        DynPullType::Up => (true, false),
        DynPullType::Down => (false, true),
        DynPullType::BusKeep => (true, true),
    };

    pin.pad_ctrl()
        .modify(|_, w| w.pue().bit(pue).pde().bit(pde));
}
