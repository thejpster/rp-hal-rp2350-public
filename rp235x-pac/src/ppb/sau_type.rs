#[doc = "Register `SAU_TYPE` reader"]
pub type R = crate::R<SAU_TYPE_SPEC>;
#[doc = "Field `SREGION` reader - The number of implemented SAU regions"]
pub type SREGION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The number of implemented SAU regions"]
    #[inline(always)]
    pub fn sregion(&self) -> SREGION_R {
        SREGION_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Indicates the number of regions implemented by the Security Attribution Unit  

You can [`read`](crate::Reg::read) this register and get [`sau_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAU_TYPE_SPEC;
impl crate::RegisterSpec for SAU_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sau_type::R`](R) reader structure"]
impl crate::Readable for SAU_TYPE_SPEC {}
#[doc = "`reset()` method sets SAU_TYPE to value 0x08"]
impl crate::Resettable for SAU_TYPE_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
