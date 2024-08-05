#[doc = "Register `SUM5` reader"]
pub type R = crate::R<SUM5_SPEC>;
#[doc = "Register `SUM5` writer"]
pub type W = crate::W<SUM5_SPEC>;
#[doc = "Field `SUM5` reader - "]
pub type SUM5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum5(&self) -> SUM5_R {
        SUM5_R::new(self.bits)
    }
}
impl W {}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sum5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM5_SPEC;
impl crate::RegisterSpec for SUM5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum5::R`](R) reader structure"]
impl crate::Readable for SUM5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sum5::W`](W) writer structure"]
impl crate::Writable for SUM5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUM5 to value 0"]
impl crate::Resettable for SUM5_SPEC {
    const RESET_VALUE: u32 = 0;
}
