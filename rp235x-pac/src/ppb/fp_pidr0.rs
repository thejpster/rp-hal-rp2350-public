#[doc = "Register `FP_PIDR0` reader"]
pub type R = crate::R<FP_PIDR0_SPEC>;
#[doc = "Field `PART_0` reader - See CoreSight Architecture Specification"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR0_SPEC;
impl crate::RegisterSpec for FP_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr0::R`](R) reader structure"]
impl crate::Readable for FP_PIDR0_SPEC {}
#[doc = "`reset()` method sets FP_PIDR0 to value 0x21"]
impl crate::Resettable for FP_PIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
