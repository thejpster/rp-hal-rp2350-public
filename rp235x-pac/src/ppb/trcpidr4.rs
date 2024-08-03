#[doc = "Register `TRCPIDR4` reader"]
pub type R = crate::R<TRCPIDR4_SPEC>;
#[doc = "Field `DES_2` reader - reads as `ImpDef"]
pub type DES_2_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - reads as `ImpDef"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as `ImpDef"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TRCPIDR4  

You can [`read`](crate::Reg::read) this register and get [`trcpidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR4_SPEC;
impl crate::RegisterSpec for TRCPIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr4::R`](R) reader structure"]
impl crate::Readable for TRCPIDR4_SPEC {}
#[doc = "`reset()` method sets TRCPIDR4 to value 0x04"]
impl crate::Resettable for TRCPIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
