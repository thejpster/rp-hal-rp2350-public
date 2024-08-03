#[doc = "Register `STREAM` reader"]
pub type R = crate::R<STREAM_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)  

You can [`read`](crate::Reg::read) this register and get [`stream::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_SPEC;
impl crate::RegisterSpec for STREAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream::R`](R) reader structure"]
impl crate::Readable for STREAM_SPEC {}
#[doc = "`reset()` method sets STREAM to value 0"]
impl crate::Resettable for STREAM_SPEC {
    const RESET_VALUE: u32 = 0;
}
