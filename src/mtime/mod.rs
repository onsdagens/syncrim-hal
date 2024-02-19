pub use embedded_hal::delay::DelayNs;

pub struct MTIME;

impl MTIME {
   pub fn new(_mtime: syncrim_pac::MTIME)-> Self {
        MTIME{}
    } 
    #[inline(always)]
    pub fn set_compare(&mut self, val: u64) {
        unsafe{
            let m = crate::pac::Peripherals::steal().MTIME;
            m.mtime_comp_lo().write(|w|w.bits(val as u32));
            m.mtime_comp_hi().write(|w|w.bits((val>>32) as u32));
        }
    }
    #[inline(always)]
    pub fn set_compare_in(&mut self, val: u64) {
        let time = self.time();
        let comp = time + val;
        self.set_compare(comp);

    }
    
    pub fn time(&self) -> u64 {
        let m = unsafe{crate::pac::Peripherals::steal().MTIME};
        let hi = m.mtime_hi().read().bits();
        let lo = m.mtime_lo().read().bits();
        //overflow check
        if m.mtime_hi().read().bits() == hi {
            (hi as u64) << 32 | lo as u64
        }
        else {
            self.time()
        }

    }
}

impl DelayNs for MTIME {
    fn delay_ns(&mut self, ns: u32) {
        let target = self.time() + ns as u64;
        while self.time() < target {
            unsafe{core::arch::asm!("nop");}
        }
    }
}
