#[doc = "Register `DWT_PIDR1` reader"]
pub type R = crate::R<DWT_PIDR1_SPEC>;
#[doc = "Field `PART_1` reader - See CoreSight Architecture Specification"]
pub type PART_1_R = crate::FieldReader;
#[doc = "Field `DES_0` reader - See CoreSight Architecture Specification"]
pub type DES_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_PIDR1_SPEC;
impl crate::RegisterSpec for DWT_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_pidr1::R`](R) reader structure"]
impl crate::Readable for DWT_PIDR1_SPEC {}
#[doc = "`reset()` method sets DWT_PIDR1 to value 0xbd"]
impl crate::Resettable for DWT_PIDR1_SPEC {
    const RESET_VALUE: u32 = 0xbd;
}
