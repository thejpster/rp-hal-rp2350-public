#[doc = "Register `SUM0` reader"]
pub type R = crate::R<SUM0_SPEC>;
#[doc = "Register `SUM0` writer"]
pub type W = crate::W<SUM0_SPEC>;
#[doc = "Field `SUM0` reader - "]
pub type SUM0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum0(&self) -> SUM0_R {
        SUM0_R::new(self.bits)
    }
}
impl W {}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sum0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM0_SPEC;
impl crate::RegisterSpec for SUM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum0::R`](R) reader structure"]
impl crate::Readable for SUM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sum0::W`](W) writer structure"]
impl crate::Writable for SUM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUM0 to value 0"]
impl crate::Resettable for SUM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
