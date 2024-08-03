#[doc = "Register `ITM_DEVTYPE` reader"]
pub type R = crate::R<ITM_DEVTYPE_SPEC>;
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
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_DEVTYPE_SPEC;
impl crate::RegisterSpec for ITM_DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_devtype::R`](R) reader structure"]
impl crate::Readable for ITM_DEVTYPE_SPEC {}
#[doc = "`reset()` method sets ITM_DEVTYPE to value 0x43"]
impl crate::Resettable for ITM_DEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0x43;
}
