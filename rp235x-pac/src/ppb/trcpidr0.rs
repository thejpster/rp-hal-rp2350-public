#[doc = "Register `TRCPIDR0` reader"]
pub type R = crate::R<TRCPIDR0_SPEC>;
#[doc = "Field `PART_0` reader - reads as `ImpDef"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TRCPIDR0  

You can [`read`](crate::Reg::read) this register and get [`trcpidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR0_SPEC;
impl crate::RegisterSpec for TRCPIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr0::R`](R) reader structure"]
impl crate::Readable for TRCPIDR0_SPEC {}
#[doc = "`reset()` method sets TRCPIDR0 to value 0x21"]
impl crate::Resettable for TRCPIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
