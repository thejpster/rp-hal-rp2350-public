#[doc = "Register `DEMCR` reader"]
pub type R = crate::R<DEMCR_SPEC>;
#[doc = "Register `DEMCR` writer"]
pub type W = crate::W<DEMCR_SPEC>;
#[doc = "Field `VC_CORERESET` reader - Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VC_CORERESET_R = crate::BitReader;
#[doc = "Field `VC_CORERESET` writer - Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VC_CORERESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_MMERR` reader - Enable halting debug trap on a MemManage exception"]
pub type VC_MMERR_R = crate::BitReader;
#[doc = "Field `VC_MMERR` writer - Enable halting debug trap on a MemManage exception"]
pub type VC_MMERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_NOCPERR` reader - Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VC_NOCPERR_R = crate::BitReader;
#[doc = "Field `VC_NOCPERR` writer - Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VC_NOCPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_CHKERR` reader - Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VC_CHKERR_R = crate::BitReader;
#[doc = "Field `VC_CHKERR` writer - Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VC_CHKERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_STATERR` reader - Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VC_STATERR_R = crate::BitReader;
#[doc = "Field `VC_STATERR` writer - Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VC_STATERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_BUSERR` reader - BusFault exception halting debug vector catch enable"]
pub type VC_BUSERR_R = crate::BitReader;
#[doc = "Field `VC_BUSERR` writer - BusFault exception halting debug vector catch enable"]
pub type VC_BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_INTERR` reader - Enable halting debug vector catch for faults during exception entry and return"]
pub type VC_INTERR_R = crate::BitReader;
#[doc = "Field `VC_INTERR` writer - Enable halting debug vector catch for faults during exception entry and return"]
pub type VC_INTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_HARDERR` reader - HardFault exception halting debug vector catch enable"]
pub type VC_HARDERR_R = crate::BitReader;
#[doc = "Field `VC_HARDERR` writer - HardFault exception halting debug vector catch enable"]
pub type VC_HARDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_SFERR` reader - SecureFault exception halting debug vector catch enable"]
pub type VC_SFERR_R = crate::BitReader;
#[doc = "Field `VC_SFERR` writer - SecureFault exception halting debug vector catch enable"]
pub type VC_SFERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_EN` reader - Enable the DebugMonitor exception"]
pub type MON_EN_R = crate::BitReader;
#[doc = "Field `MON_EN` writer - Enable the DebugMonitor exception"]
pub type MON_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_PEND` reader - Sets or clears the pending state of the DebugMonitor exception"]
pub type MON_PEND_R = crate::BitReader;
#[doc = "Field `MON_PEND` writer - Sets or clears the pending state of the DebugMonitor exception"]
pub type MON_PEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_STEP` reader - Enable DebugMonitor stepping"]
pub type MON_STEP_R = crate::BitReader;
#[doc = "Field `MON_STEP` writer - Enable DebugMonitor stepping"]
pub type MON_STEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_REQ` reader - DebugMonitor semaphore bit"]
pub type MON_REQ_R = crate::BitReader;
#[doc = "Field `MON_REQ` writer - DebugMonitor semaphore bit"]
pub type MON_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDME` reader - Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
pub type SDME_R = crate::BitReader;
#[doc = "Field `TRCENA` reader - Global enable for all DWT and ITM features"]
pub type TRCENA_R = crate::BitReader;
#[doc = "Field `TRCENA` writer - Global enable for all DWT and ITM features"]
pub type TRCENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_sferr(&self) -> VC_SFERR_R {
        VC_SFERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DebugMonitor stepping"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DebugMonitor semaphore bit"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
    #[inline(always)]
    pub fn sdme(&self) -> SDME_R {
        SDME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Global enable for all DWT and ITM features"]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
    #[inline(always)]
    #[must_use]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W<DEMCR_SPEC> {
        VC_CORERESET_W::new(self, 0)
    }
    #[doc = "Bit 4 - Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W<DEMCR_SPEC> {
        VC_MMERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W<DEMCR_SPEC> {
        VC_NOCPERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W<DEMCR_SPEC> {
        VC_CHKERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W<DEMCR_SPEC> {
        VC_STATERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W<DEMCR_SPEC> {
        VC_BUSERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VC_INTERR_W<DEMCR_SPEC> {
        VC_INTERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W<DEMCR_SPEC> {
        VC_HARDERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_sferr(&mut self) -> VC_SFERR_W<DEMCR_SPEC> {
        VC_SFERR_W::new(self, 11)
    }
    #[doc = "Bit 16 - Enable the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MON_EN_W<DEMCR_SPEC> {
        MON_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MON_PEND_W<DEMCR_SPEC> {
        MON_PEND_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable DebugMonitor stepping"]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MON_STEP_W<DEMCR_SPEC> {
        MON_STEP_W::new(self, 18)
    }
    #[doc = "Bit 19 - DebugMonitor semaphore bit"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MON_REQ_W<DEMCR_SPEC> {
        MON_REQ_W::new(self, 19)
    }
    #[doc = "Bit 24 - Global enable for all DWT and ITM features"]
    #[inline(always)]
    #[must_use]
    pub fn trcena(&mut self) -> TRCENA_W<DEMCR_SPEC> {
        TRCENA_W::new(self, 24)
    }
}
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging  

You can [`read`](crate::Reg::read) this register and get [`demcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEMCR_SPEC;
impl crate::RegisterSpec for DEMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demcr::R`](R) reader structure"]
impl crate::Readable for DEMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`demcr::W`](W) writer structure"]
impl crate::Writable for DEMCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DEMCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
