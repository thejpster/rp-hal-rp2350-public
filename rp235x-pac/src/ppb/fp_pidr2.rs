#[doc = "Register `FP_PIDR2` reader"]
pub type R = crate::R<FP_PIDR2_SPEC>;
#[doc = "Field `DES_1` reader - See CoreSight Architecture Specification"]
pub type DES_1_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - See CoreSight Architecture Specification"]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - See CoreSight Architecture Specification"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR2_SPEC;
impl crate::RegisterSpec for FP_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr2::R`](R) reader structure"]
impl crate::Readable for FP_PIDR2_SPEC {}
#[doc = "`reset()` method sets FP_PIDR2 to value 0x0b"]
impl crate::Resettable for FP_PIDR2_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
