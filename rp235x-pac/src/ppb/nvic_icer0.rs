#[doc = "Register `NVIC_ICER0` reader"]
pub type R = crate::R<NVIC_ICER0_SPEC>;
#[doc = "Register `NVIC_ICER0` writer"]
pub type W = crate::W<NVIC_ICER0_SPEC>;
#[doc = "Field `CLRENA` reader - For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type CLRENA_R = crate::FieldReader<u32>;
#[doc = "Field `CLRENA` writer - For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type CLRENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clrena(&mut self) -> CLRENA_W<NVIC_ICER0_SPEC> {
        CLRENA_W::new(self, 0)
    }
}
#[doc = "Clears or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_icer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ICER0_SPEC;
impl crate::RegisterSpec for NVIC_ICER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icer0::R`](R) reader structure"]
impl crate::Readable for NVIC_ICER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_icer0::W`](W) writer structure"]
impl crate::Writable for NVIC_ICER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ICER0 to value 0"]
impl crate::Resettable for NVIC_ICER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
