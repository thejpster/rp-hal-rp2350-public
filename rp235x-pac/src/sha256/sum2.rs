#[doc = "Register `SUM2` reader"]
pub type R = crate::R<SUM2_SPEC>;
#[doc = "Register `SUM2` writer"]
pub type W = crate::W<SUM2_SPEC>;
#[doc = "Field `SUM2` reader - "]
pub type SUM2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum2(&self) -> SUM2_R {
        SUM2_R::new(self.bits)
    }
}
impl W {}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sum2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM2_SPEC;
impl crate::RegisterSpec for SUM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum2::R`](R) reader structure"]
impl crate::Readable for SUM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sum2::W`](W) writer structure"]
impl crate::Writable for SUM2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUM2 to value 0"]
impl crate::Resettable for SUM2_SPEC {
    const RESET_VALUE: u32 = 0;
}
