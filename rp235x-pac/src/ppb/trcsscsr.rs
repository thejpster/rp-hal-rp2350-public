#[doc = "Register `TRCSSCSR` reader"]
pub type R = crate::R<TRCSSCSR_SPEC>;
#[doc = "Register `TRCSSCSR` writer"]
pub type W = crate::W<TRCSSCSR_SPEC>;
#[doc = "Field `INST` reader - Reserved, RES0"]
pub type INST_R = crate::BitReader;
#[doc = "Field `DA` reader - Reserved, RES0"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DV` reader - Reserved, RES0"]
pub type DV_R = crate::BitReader;
#[doc = "Field `PC` reader - Reserved, RES1"]
pub type PC_R = crate::BitReader;
#[doc = "Field `STATUS` reader - Single-shot status bit. Indicates if any of the comparators, that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects, have matched"]
pub type STATUS_R = crate::BitReader;
#[doc = "Field `STATUS` writer - Single-shot status bit. Indicates if any of the comparators, that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects, have matched"]
pub type STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved, RES0"]
    #[inline(always)]
    pub fn inst(&self) -> INST_R {
        INST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved, RES0"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved, RES0"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved, RES1"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Single-shot status bit. Indicates if any of the comparators, that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects, have matched"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Single-shot status bit. Indicates if any of the comparators, that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects, have matched"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<TRCSSCSR_SPEC> {
        STATUS_W::new(self, 31)
    }
}
#[doc = "Controls the corresponding single-shot comparator resource  

You can [`read`](crate::Reg::read) this register and get [`trcsscsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsscsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCSSCSR_SPEC;
impl crate::RegisterSpec for TRCSSCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsscsr::R`](R) reader structure"]
impl crate::Readable for TRCSSCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcsscsr::W`](W) writer structure"]
impl crate::Writable for TRCSSCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCSSCSR to value 0"]
impl crate::Resettable for TRCSSCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
