#[doc = "Register `DHCSR` reader"]
pub type R = crate::R<DHCSR_SPEC>;
#[doc = "Register `DHCSR` writer"]
pub type W = crate::W<DHCSR_SPEC>;
#[doc = "Field `C_DEBUGEN` reader - Enable Halting debug"]
pub type C_DEBUGEN_R = crate::BitReader;
#[doc = "Field `C_DEBUGEN` writer - Enable Halting debug"]
pub type C_DEBUGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_HALT` reader - PE enter Debug state halt request"]
pub type C_HALT_R = crate::BitReader;
#[doc = "Field `C_HALT` writer - PE enter Debug state halt request"]
pub type C_HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_STEP` reader - Enable single instruction step"]
pub type C_STEP_R = crate::BitReader;
#[doc = "Field `C_STEP` writer - Enable single instruction step"]
pub type C_STEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_MASKINTS` reader - When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_R = crate::BitReader;
#[doc = "Field `C_MASKINTS` writer - When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_SNAPSTALL` reader - Allow imprecise entry to Debug state"]
pub type C_SNAPSTALL_R = crate::BitReader;
#[doc = "Field `C_SNAPSTALL` writer - Allow imprecise entry to Debug state"]
pub type C_SNAPSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_REGRDY` reader - Handshake flag to transfers through the DCRDR"]
pub type S_REGRDY_R = crate::BitReader;
#[doc = "Field `S_HALT` reader - Indicates whether the PE is in Debug state"]
pub type S_HALT_R = crate::BitReader;
#[doc = "Field `S_SLEEP` reader - Indicates whether the PE is sleeping"]
pub type S_SLEEP_R = crate::BitReader;
#[doc = "Field `S_LOCKUP` reader - Indicates whether the PE is in Lockup state"]
pub type S_LOCKUP_R = crate::BitReader;
#[doc = "Field `S_SDE` reader - Indicates whether Secure invasive debug is allowed"]
pub type S_SDE_R = crate::BitReader;
#[doc = "Field `S_RETIRE_ST` reader - Set to 1 every time the PE retires one of more instructions"]
pub type S_RETIRE_ST_R = crate::BitReader;
#[doc = "Field `S_RESET_ST` reader - Indicates whether the PE has been reset since the last read of the DHCSR"]
pub type S_RESET_ST_R = crate::BitReader;
#[doc = "Field `S_RESTART_ST` reader - Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
pub type S_RESTART_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable Halting debug"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PE enter Debug state halt request"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable single instruction step"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow imprecise entry to Debug state"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Handshake flag to transfers through the DCRDR"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates whether the PE is in Debug state"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates whether the PE is sleeping"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates whether the PE is in Lockup state"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates whether Secure invasive debug is allowed"]
    #[inline(always)]
    pub fn s_sde(&self) -> S_SDE_R {
        S_SDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set to 1 every time the PE retires one of more instructions"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates whether the PE has been reset since the last read of the DHCSR"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
    #[inline(always)]
    pub fn s_restart_st(&self) -> S_RESTART_ST_R {
        S_RESTART_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Halting debug"]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W<DHCSR_SPEC> {
        C_DEBUGEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PE enter Debug state halt request"]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> C_HALT_W<DHCSR_SPEC> {
        C_HALT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable single instruction step"]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> C_STEP_W<DHCSR_SPEC> {
        C_STEP_W::new(self, 2)
    }
    #[doc = "Bit 3 - When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W<DHCSR_SPEC> {
        C_MASKINTS_W::new(self, 3)
    }
    #[doc = "Bit 5 - Allow imprecise entry to Debug state"]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W<DHCSR_SPEC> {
        C_SNAPSTALL_W::new(self, 5)
    }
}
#[doc = "Controls halting debug  

You can [`read`](crate::Reg::read) this register and get [`dhcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhcsr::R`](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhcsr::W`](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
