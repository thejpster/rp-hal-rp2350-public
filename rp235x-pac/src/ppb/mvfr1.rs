#[doc = "Register `MVFR1` reader"]
pub type R = crate::R<MVFR1_SPEC>;
#[doc = "Register `MVFR1` writer"]
pub type W = crate::W<MVFR1_SPEC>;
#[doc = "Field `FPFTZ` reader - Indicates whether subnormals are always flushed-to-zero"]
pub type FPFTZ_R = crate::FieldReader;
#[doc = "Field `FPDNAN` reader - Indicates whether the FP hardware implementation supports NaN propagation"]
pub type FPDNAN_R = crate::FieldReader;
#[doc = "Field `FPHP` reader - Indicates whether the FP Extension implements half-precision FP conversion instructions"]
pub type FPHP_R = crate::FieldReader;
#[doc = "Field `FMAC` reader - Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
pub type FMAC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates whether subnormals are always flushed-to-zero"]
    #[inline(always)]
    pub fn fpftz(&self) -> FPFTZ_R {
        FPFTZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates whether the FP hardware implementation supports NaN propagation"]
    #[inline(always)]
    pub fn fpdnan(&self) -> FPDNAN_R {
        FPDNAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates whether the FP Extension implements half-precision FP conversion instructions"]
    #[inline(always)]
    pub fn fphp(&self) -> FPHP_R {
        FPHP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
    #[inline(always)]
    pub fn fmac(&self) -> FMAC_R {
        FMAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVFR1_SPEC;
impl crate::RegisterSpec for MVFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr1::R`](R) reader structure"]
impl crate::Readable for MVFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mvfr1::W`](W) writer structure"]
impl crate::Writable for MVFR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR1 to value 0x8500_0089"]
impl crate::Resettable for MVFR1_SPEC {
    const RESET_VALUE: u32 = 0x8500_0089;
}
