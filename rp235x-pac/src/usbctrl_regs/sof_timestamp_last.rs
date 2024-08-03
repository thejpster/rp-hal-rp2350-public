#[doc = "Register `SOF_TIMESTAMP_LAST` reader"]
pub type R = crate::R<SOF_TIMESTAMP_LAST_SPEC>;
#[doc = "Field `SOF_TIMESTAMP_LAST` reader - "]
pub type SOF_TIMESTAMP_LAST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:20"]
    #[inline(always)]
    pub fn sof_timestamp_last(&self) -> SOF_TIMESTAMP_LAST_R {
        SOF_TIMESTAMP_LAST_R::new(self.bits & 0x001f_ffff)
    }
}
#[doc = "Device only. Value of free-running PHY clock counter @48MHz when last SOF event occured.  

You can [`read`](crate::Reg::read) this register and get [`sof_timestamp_last::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_TIMESTAMP_LAST_SPEC;
impl crate::RegisterSpec for SOF_TIMESTAMP_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof_timestamp_last::R`](R) reader structure"]
impl crate::Readable for SOF_TIMESTAMP_LAST_SPEC {}
#[doc = "`reset()` method sets SOF_TIMESTAMP_LAST to value 0"]
impl crate::Resettable for SOF_TIMESTAMP_LAST_SPEC {
    const RESET_VALUE: u32 = 0;
}
