#[doc = "Register `SUM1` reader"]
pub type R = crate::R<SUM1_SPEC>;
#[doc = "Register `SUM1` writer"]
pub type W = crate::W<SUM1_SPEC>;
#[doc = "Field `SUM1` reader - "]
pub type SUM1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum1(&self) -> SUM1_R {
        SUM1_R::new(self.bits)
    }
}
impl W {}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sum1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM1_SPEC;
impl crate::RegisterSpec for SUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum1::R`](R) reader structure"]
impl crate::Readable for SUM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sum1::W`](W) writer structure"]
impl crate::Writable for SUM1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUM1 to value 0"]
impl crate::Resettable for SUM1_SPEC {
    const RESET_VALUE: u32 = 0;
}
