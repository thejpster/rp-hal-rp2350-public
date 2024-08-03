#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ACTLR_SPEC>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ACTLR_SPEC>;
#[doc = "Field `DISMCYCINT` reader - Disable dual-issue."]
pub type DISMCYCINT_R = crate::BitReader;
#[doc = "Field `DISMCYCINT` writer - Disable dual-issue."]
pub type DISMCYCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFOLD` reader - Disable dual-issue."]
pub type DISFOLD_R = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disable dual-issue."]
pub type DISFOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISOOFP` reader - Disable out-of-order FP instruction completion"]
pub type DISOOFP_R = crate::BitReader;
#[doc = "Field `DISOOFP` writer - Disable out-of-order FP instruction completion"]
pub type DISOOFP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEXCODIS` reader - Disable FPU exception outputs"]
pub type FPEXCODIS_R = crate::BitReader;
#[doc = "Field `FPEXCODIS` writer - Disable FPU exception outputs"]
pub type FPEXCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISITMATBFLUSH` reader - Disable ATB Flush"]
pub type DISITMATBFLUSH_R = crate::BitReader;
#[doc = "Field `DISITMATBFLUSH` writer - Disable ATB Flush"]
pub type DISITMATBFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTEXCLALL` reader - External Exclusives Allowed with no MPU"]
pub type EXTEXCLALL_R = crate::BitReader;
#[doc = "Field `EXTEXCLALL` writer - External Exclusives Allowed with no MPU"]
pub type EXTEXCLALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable dual-issue."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Disable dual-issue."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instruction completion"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable ATB Flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 29 - External Exclusives Allowed with no MPU"]
    #[inline(always)]
    pub fn extexclall(&self) -> EXTEXCLALL_R {
        EXTEXCLALL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<ACTLR_SPEC> {
        DISMCYCINT_W::new(self, 0)
    }
    #[doc = "Bit 2 - Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<ACTLR_SPEC> {
        DISFOLD_W::new(self, 2)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DISOOFP_W<ACTLR_SPEC> {
        DISOOFP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Disable FPU exception outputs"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<ACTLR_SPEC> {
        FPEXCODIS_W::new(self, 10)
    }
    #[doc = "Bit 12 - Disable ATB Flush"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<ACTLR_SPEC> {
        DISITMATBFLUSH_W::new(self, 12)
    }
    #[doc = "Bit 29 - External Exclusives Allowed with no MPU"]
    #[inline(always)]
    #[must_use]
    pub fn extexclall(&mut self) -> EXTEXCLALL_W<ACTLR_SPEC> {
        EXTEXCLALL_W::new(self, 29)
    }
}
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options  

You can [`read`](crate::Reg::read) this register and get [`actlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
