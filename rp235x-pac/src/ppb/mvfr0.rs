#[doc = "Register `MVFR0` reader"]
pub type R = crate::R<MVFR0_SPEC>;
#[doc = "Register `MVFR0` writer"]
pub type W = crate::W<MVFR0_SPEC>;
#[doc = "Field `SIMDREG` reader - Indicates size of FP register file"]
pub type SIMDREG_R = crate::FieldReader;
#[doc = "Field `FPSP` reader - Indicates support for FP single-precision operations"]
pub type FPSP_R = crate::FieldReader;
#[doc = "Field `FPDP` reader - Indicates support for FP double-precision operations"]
pub type FPDP_R = crate::FieldReader;
#[doc = "Field `FPDIVIDE` reader - Indicates the support for FP divide operations"]
pub type FPDIVIDE_R = crate::FieldReader;
#[doc = "Field `FPSQRT` reader - Indicates the support for FP square root operations"]
pub type FPSQRT_R = crate::FieldReader;
#[doc = "Field `FPROUND` reader - Indicates the rounding modes supported by the FP Extension"]
pub type FPROUND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates size of FP register file"]
    #[inline(always)]
    pub fn simdreg(&self) -> SIMDREG_R {
        SIMDREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates support for FP single-precision operations"]
    #[inline(always)]
    pub fn fpsp(&self) -> FPSP_R {
        FPSP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates support for FP double-precision operations"]
    #[inline(always)]
    pub fn fpdp(&self) -> FPDP_R {
        FPDP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the support for FP divide operations"]
    #[inline(always)]
    pub fn fpdivide(&self) -> FPDIVIDE_R {
        FPDIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the support for FP square root operations"]
    #[inline(always)]
    pub fn fpsqrt(&self) -> FPSQRT_R {
        FPSQRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates the rounding modes supported by the FP Extension"]
    #[inline(always)]
    pub fn fpround(&self) -> FPROUND_R {
        FPROUND_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVFR0_SPEC;
impl crate::RegisterSpec for MVFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr0::R`](R) reader structure"]
impl crate::Readable for MVFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mvfr0::W`](W) writer structure"]
impl crate::Writable for MVFR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR0 to value 0x6054_0601"]
impl crate::Resettable for MVFR0_SPEC {
    const RESET_VALUE: u32 = 0x6054_0601;
}
