#[doc = "Register `TRCCCCTLR` reader"]
pub type R = crate::R<TRCCCCTLR_SPEC>;
#[doc = "Register `TRCCCCTLR` writer"]
pub type W = crate::W<TRCCCCTLR_SPEC>;
#[doc = "Field `THRESHOLD` reader - Instruction trace cycle count threshold"]
pub type THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `THRESHOLD` writer - Instruction trace cycle count threshold"]
pub type THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Instruction trace cycle count threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Instruction trace cycle count threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<TRCCCCTLR_SPEC> {
        THRESHOLD_W::new(self, 0)
    }
}
#[doc = "The TRCCCCTLR sets the threshold value for instruction trace cycle counting. The threshold represents the minimum interval between cycle count trace packets  

You can [`read`](crate::Reg::read) this register and get [`trcccctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcccctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCCCTLR_SPEC;
impl crate::RegisterSpec for TRCCCCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcccctlr::R`](R) reader structure"]
impl crate::Readable for TRCCCCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcccctlr::W`](W) writer structure"]
impl crate::Writable for TRCCCCTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCCCTLR to value 0"]
impl crate::Resettable for TRCCCCTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
