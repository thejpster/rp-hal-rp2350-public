#[doc = "Register `TRCDEVTYPE` reader"]
pub type R = crate::R<TRCDEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - reads as 0b0011"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - reads as 0b0001"]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as 0b0011"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as 0b0001"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TRCDEVTYPE  

You can [`read`](crate::Reg::read) this register and get [`trcdevtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCDEVTYPE_SPEC;
impl crate::RegisterSpec for TRCDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevtype::R`](R) reader structure"]
impl crate::Readable for TRCDEVTYPE_SPEC {}
#[doc = "`reset()` method sets TRCDEVTYPE to value 0x13"]
impl crate::Resettable for TRCDEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0x13;
}
