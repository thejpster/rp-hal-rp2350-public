#[doc = "Register `DDEVTYPE` reader"]
pub type R = crate::R<DDEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - CoreSight major type"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - Component sub-type"]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CoreSight major type"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component sub-type"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`ddevtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDEVTYPE_SPEC;
impl crate::RegisterSpec for DDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddevtype::R`](R) reader structure"]
impl crate::Readable for DDEVTYPE_SPEC {}
#[doc = "`reset()` method sets DDEVTYPE to value 0"]
impl crate::Resettable for DDEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
