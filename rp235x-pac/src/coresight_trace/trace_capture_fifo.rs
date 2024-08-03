#[doc = "Register `TRACE_CAPTURE_FIFO` reader"]
pub type R = crate::R<TRACE_CAPTURE_FIFO_SPEC>;
#[doc = "Field `RDATA` reader - captured trace data"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - captured trace data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "trace capture fifo  

You can [`read`](crate::Reg::read) this register and get [`trace_capture_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_CAPTURE_FIFO_SPEC;
impl crate::RegisterSpec for TRACE_CAPTURE_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_capture_fifo::R`](R) reader structure"]
impl crate::Readable for TRACE_CAPTURE_FIFO_SPEC {}
#[doc = "`reset()` method sets TRACE_CAPTURE_FIFO to value 0"]
impl crate::Resettable for TRACE_CAPTURE_FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
