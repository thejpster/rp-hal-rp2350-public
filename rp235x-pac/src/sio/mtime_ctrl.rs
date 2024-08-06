#[doc = "Register `MTIME_CTRL` reader"]
pub type R = crate::R<MTIME_CTRL_SPEC>;
#[doc = "Register `MTIME_CTRL` writer"]
pub type W = crate::W<MTIME_CTRL_SPEC>;
#[doc = "Field `EN` reader - Timer enable bit. When 0, the timer will not increment automatically."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Timer enable bit. When 0, the timer will not increment automatically."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLSPEED` reader - If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
pub type FULLSPEED_R = crate::BitReader;
#[doc = "Field `FULLSPEED` writer - If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
pub type FULLSPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGPAUSE_CORE0` reader - If 1, the timer pauses when core 0 is in the debug halt state."]
pub type DBGPAUSE_CORE0_R = crate::BitReader;
#[doc = "Field `DBGPAUSE_CORE0` writer - If 1, the timer pauses when core 0 is in the debug halt state."]
pub type DBGPAUSE_CORE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGPAUSE_CORE1` reader - If 1, the timer pauses when core 1 is in the debug halt state."]
pub type DBGPAUSE_CORE1_R = crate::BitReader;
#[doc = "Field `DBGPAUSE_CORE1` writer - If 1, the timer pauses when core 1 is in the debug halt state."]
pub type DBGPAUSE_CORE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable bit. When 0, the timer will not increment automatically."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
    #[inline(always)]
    pub fn fullspeed(&self) -> FULLSPEED_R {
        FULLSPEED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, the timer pauses when core 0 is in the debug halt state."]
    #[inline(always)]
    pub fn dbgpause_core0(&self) -> DBGPAUSE_CORE0_R {
        DBGPAUSE_CORE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, the timer pauses when core 1 is in the debug halt state."]
    #[inline(always)]
    pub fn dbgpause_core1(&self) -> DBGPAUSE_CORE1_R {
        DBGPAUSE_CORE1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable bit. When 0, the timer will not increment automatically."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MTIME_CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
    #[inline(always)]
    #[must_use]
    pub fn fullspeed(&mut self) -> FULLSPEED_W<MTIME_CTRL_SPEC> {
        FULLSPEED_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, the timer pauses when core 0 is in the debug halt state."]
    #[inline(always)]
    #[must_use]
    pub fn dbgpause_core0(&mut self) -> DBGPAUSE_CORE0_W<MTIME_CTRL_SPEC> {
        DBGPAUSE_CORE0_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, the timer pauses when core 1 is in the debug halt state."]
    #[inline(always)]
    #[must_use]
    pub fn dbgpause_core1(&mut self) -> DBGPAUSE_CORE1_W<MTIME_CTRL_SPEC> {
        DBGPAUSE_CORE1_W::new(self, 3)
    }
}
#[doc = "Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores.  

You can [`read`](crate::Reg::read) this register and get [`mtime_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_CTRL_SPEC;
impl crate::RegisterSpec for MTIME_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_ctrl::R`](R) reader structure"]
impl crate::Readable for MTIME_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtime_ctrl::W`](W) writer structure"]
impl crate::Writable for MTIME_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIME_CTRL to value 0x0d"]
impl crate::Resettable for MTIME_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
