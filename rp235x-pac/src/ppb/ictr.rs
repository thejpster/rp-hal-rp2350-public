#[doc = "Register `ICTR` reader"]
pub type R = crate::R<ICTR_SPEC>;
#[doc = "Register `ICTR` writer"]
pub type W = crate::W<ICTR_SPEC>;
#[doc = "Field `INTLINESNUM` reader - Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4×INTLINESNUM"]
pub type INTLINESNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4×INTLINESNUM"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the interrupt controller  

You can [`read`](crate::Reg::read) this register and get [`ictr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ictr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICTR_SPEC;
impl crate::RegisterSpec for ICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictr::R`](R) reader structure"]
impl crate::Readable for ICTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ictr::W`](W) writer structure"]
impl crate::Writable for ICTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICTR to value 0x01"]
impl crate::Resettable for ICTR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
