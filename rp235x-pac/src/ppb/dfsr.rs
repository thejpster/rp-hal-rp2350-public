#[doc = "Register `DFSR` reader"]
pub type R = crate::R<DFSR_SPEC>;
#[doc = "Register `DFSR` writer"]
pub type W = crate::W<DFSR_SPEC>;
#[doc = "Field `HALTED` reader - Sticky flag indicating that a Halt request debug event or Step debug event has occurred"]
pub type HALTED_R = crate::BitReader;
#[doc = "Field `HALTED` writer - Sticky flag indicating that a Halt request debug event or Step debug event has occurred"]
pub type HALTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPT` reader - Sticky flag indicating whether a Breakpoint debug event has occurred"]
pub type BKPT_R = crate::BitReader;
#[doc = "Field `BKPT` writer - Sticky flag indicating whether a Breakpoint debug event has occurred"]
pub type BKPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWTTRAP` reader - Sticky flag indicating whether a Watchpoint debug event has occurred"]
pub type DWTTRAP_R = crate::BitReader;
#[doc = "Field `DWTTRAP` writer - Sticky flag indicating whether a Watchpoint debug event has occurred"]
pub type DWTTRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCATCH` reader - Sticky flag indicating whether a Vector catch debug event has occurred"]
pub type VCATCH_R = crate::BitReader;
#[doc = "Field `VCATCH` writer - Sticky flag indicating whether a Vector catch debug event has occurred"]
pub type VCATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL` reader - Sticky flag indicating whether an External debug request debug event has occurred"]
pub type EXTERNAL_R = crate::BitReader;
#[doc = "Field `EXTERNAL` writer - Sticky flag indicating whether an External debug request debug event has occurred"]
pub type EXTERNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sticky flag indicating that a Halt request debug event or Step debug event has occurred"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky flag indicating whether a Breakpoint debug event has occurred"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sticky flag indicating whether a Watchpoint debug event has occurred"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sticky flag indicating whether a Vector catch debug event has occurred"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sticky flag indicating whether an External debug request debug event has occurred"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky flag indicating that a Halt request debug event or Step debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HALTED_W<DFSR_SPEC> {
        HALTED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sticky flag indicating whether a Breakpoint debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BKPT_W<DFSR_SPEC> {
        BKPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sticky flag indicating whether a Watchpoint debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DWTTRAP_W<DFSR_SPEC> {
        DWTTRAP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Sticky flag indicating whether a Vector catch debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VCATCH_W<DFSR_SPEC> {
        VCATCH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Sticky flag indicating whether an External debug request debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<DFSR_SPEC> {
        EXTERNAL_W::new(self, 4)
    }
}
#[doc = "Shows which debug event occurred  

You can [`read`](crate::Reg::read) this register and get [`dfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsr::R`](R) reader structure"]
impl crate::Readable for DFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfsr::W`](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
