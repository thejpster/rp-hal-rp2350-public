#[doc = "Register `DWT_DEVTYPE` reader"]
pub type R = crate::R<DWT_DEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - Component major type"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - Component sub-type"]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Component major type"]
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
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_DEVTYPE_SPEC;
impl crate::RegisterSpec for DWT_DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_devtype::R`](R) reader structure"]
impl crate::Readable for DWT_DEVTYPE_SPEC {}
#[doc = "`reset()` method sets DWT_DEVTYPE to value 0"]
impl crate::Resettable for DWT_DEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
