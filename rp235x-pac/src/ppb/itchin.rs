#[doc = "Register `ITCHIN` reader"]
pub type R = crate::R<ITCHIN_SPEC>;
#[doc = "Field `CTCHIN` reader - Reads the value of the ctichin inputs."]
pub type CTCHIN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reads the value of the ctichin inputs."]
    #[inline(always)]
    pub fn ctchin(&self) -> CTCHIN_R {
        CTCHIN_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Integration Test Channel Input register  

You can [`read`](crate::Reg::read) this register and get [`itchin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCHIN_SPEC;
impl crate::RegisterSpec for ITCHIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itchin::R`](R) reader structure"]
impl crate::Readable for ITCHIN_SPEC {}
#[doc = "`reset()` method sets ITCHIN to value 0"]
impl crate::Resettable for ITCHIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
