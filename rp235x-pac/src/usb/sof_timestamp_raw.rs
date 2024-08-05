#[doc = "Register `SOF_TIMESTAMP_RAW` reader"]
pub type R = crate::R<SOF_TIMESTAMP_RAW_SPEC>;
#[doc = "Register `SOF_TIMESTAMP_RAW` writer"]
pub type W = crate::W<SOF_TIMESTAMP_RAW_SPEC>;
#[doc = "Field `SOF_TIMESTAMP_RAW` reader - "]
pub type SOF_TIMESTAMP_RAW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:20"]
    #[inline(always)]
    pub fn sof_timestamp_raw(&self) -> SOF_TIMESTAMP_RAW_R {
        SOF_TIMESTAMP_RAW_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {}
#[doc = "Device only. Raw value of free-running PHY clock counter @48MHz. Used to calculate time between SOF events.  

You can [`read`](crate::Reg::read) this register and get [`sof_timestamp_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sof_timestamp_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_TIMESTAMP_RAW_SPEC;
impl crate::RegisterSpec for SOF_TIMESTAMP_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof_timestamp_raw::R`](R) reader structure"]
impl crate::Readable for SOF_TIMESTAMP_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sof_timestamp_raw::W`](W) writer structure"]
impl crate::Writable for SOF_TIMESTAMP_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOF_TIMESTAMP_RAW to value 0"]
impl crate::Resettable for SOF_TIMESTAMP_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
