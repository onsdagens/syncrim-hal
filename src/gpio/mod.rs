use core::marker::PhantomData;

pub use embedded_hal::digital::{self, *};
pub type Error = core::convert::Infallible;

pub struct Pin<T> {
    number: u8,
    _marker: PhantomData<T>,
}

pub struct Pins {
    pub pin0: Pin<FreshPin>,
    pub pin1: Pin<FreshPin>,
    pub pin2: Pin<FreshPin>,
    pub pin3: Pin<FreshPin>,
    pub pin4: Pin<FreshPin>,
}

pub struct Output;
pub struct FreshPin;
//struct Input;
impl Pins {
    pub fn new(_gpio: syncrim_pac::GPIO) -> Pins {
        Pins {
            pin0: Pin {
                number: 0,
                _marker: PhantomData,
            },
            pin1: Pin {
                number: 1,
                _marker: PhantomData,
            },
            pin2: Pin {
                number: 2,
                _marker: PhantomData,
            },
            pin3: Pin {
                number: 3,
                _marker: PhantomData,
            },
            pin4: Pin {
                number: 4,
                _marker: PhantomData,
            },
        }
    }
}
impl<T> Pin<T> {
    #[inline(always)]
    pub fn into_output(self) -> Pin<Output> {
        let g = unsafe { crate::pac::Peripherals::steal().GPIO };
        let e = g.enable().read().bits();
        let c = g.config().read().bits();
        unsafe {
            g.enable().write(|w| w.bits(e | 1 << self.number));
            g.config().write(|w| w.bits(c ^ 1 << self.number));
        }
        Pin {
            number: self.number,
            _marker: PhantomData,
        }
    }
}
impl digital::OutputPin for Pin<Output> {
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            crate::pac::Peripherals::steal()
                .GPIO
                .clear()
                .write(|w| w.bits(1 << self.number))
        };
        Ok(())
    }
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            crate::pac::Peripherals::steal()
                .GPIO
                .set()
                .write(|w| w.bits(1 << self.number))
        };
        Ok(())
    }
    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        match state {
            PinState::Low => return self.set_low(),
            PinState::High => return self.set_high(),
        }
    }
}

impl digital::StatefulOutputPin for Pin<Output> {
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            crate::pac::Peripherals::steal()
                .GPIO
                .toggle()
                .write(|w| w.bits(1 << self.number))
        }
        Ok(())
    }
}
impl<T> ErrorType for Pin<T> {
    type Error = Error;
}
