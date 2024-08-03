#[doc = "Register `TRCPIDR2` reader"]
pub type R = crate::R<TRCPIDR2_SPEC>;
#[doc = "Field `DES_0` reader - reads as `ImpDef"]
pub type DES_0_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - reads as 0b1"]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - reads as `ImpDef"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - reads as `ImpDef"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reads as 0b1"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TRCPIDR2  

You can [`read`](crate::Reg::read) this register and get [`trcpidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR2_SPEC;
impl crate::RegisterSpec for TRCPIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr2::R`](R) reader structure"]
impl crate::Readable for TRCPIDR2_SPEC {}
#[doc = "`reset()` method sets TRCPIDR2 to value 0x2b"]
impl crate::Resettable for TRCPIDR2_SPEC {
    const RESET_VALUE: u32 = 0x2b;
}
