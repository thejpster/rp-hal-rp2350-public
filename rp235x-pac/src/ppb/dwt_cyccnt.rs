#[doc = "Register `DWT_CYCCNT` reader"]
pub type R = crate::R<DWT_CYCCNT_SPEC>;
#[doc = "Register `DWT_CYCCNT` writer"]
pub type W = crate::W<DWT_CYCCNT_SPEC>;
#[doc = "Field `CYCCNT` reader - Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CYCCNT_R = crate::FieldReader<u32>;
#[doc = "Field `CYCCNT` writer - Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CYCCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    pub fn cyccnt(&self) -> CYCCNT_R {
        CYCCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cyccnt(&mut self) -> CYCCNT_W<DWT_CYCCNT_SPEC> {
        CYCCNT_W::new(self, 0)
    }
}
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cyccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cyccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_CYCCNT_SPEC;
impl crate::RegisterSpec for DWT_CYCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_cyccnt::R`](R) reader structure"]
impl crate::Readable for DWT_CYCCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_cyccnt::W`](W) writer structure"]
impl crate::Writable for DWT_CYCCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_CYCCNT to value 0"]
impl crate::Resettable for DWT_CYCCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
