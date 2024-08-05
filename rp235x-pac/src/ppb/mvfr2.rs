#[doc = "Register `MVFR2` reader"]
pub type R = crate::R<MVFR2_SPEC>;
#[doc = "Register `MVFR2` writer"]
pub type W = crate::W<MVFR2_SPEC>;
#[doc = "Field `FPMISC` reader - Indicates support for miscellaneous FP features"]
pub type FPMISC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Indicates support for miscellaneous FP features"]
    #[inline(always)]
    pub fn fpmisc(&self) -> FPMISC_R {
        FPMISC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVFR2_SPEC;
impl crate::RegisterSpec for MVFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr2::R`](R) reader structure"]
impl crate::Readable for MVFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mvfr2::W`](W) writer structure"]
impl crate::Writable for MVFR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR2 to value 0x60"]
impl crate::Resettable for MVFR2_SPEC {
    const RESET_VALUE: u32 = 0x60;
}
