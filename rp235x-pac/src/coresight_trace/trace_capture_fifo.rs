#[doc = "Register `TRACE_CAPTURE_FIFO` reader"]
pub type R = crate::R<TRACE_CAPTURE_FIFO_SPEC>;
#[doc = "Register `TRACE_CAPTURE_FIFO` writer"]
pub type W = crate::W<TRACE_CAPTURE_FIFO_SPEC>;
#[doc = "Field `RDATA` reader - Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true: * TPIU TRACECTL output is low (normal trace data) * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet) These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock.  

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true: * TPIU TRACECTL output is low (normal trace data) * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet) These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
impl W {}
#[doc = "FIFO for trace data captured from the TPIU  

You can [`read`](crate::Reg::read) this register and get [`trace_capture_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_capture_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_CAPTURE_FIFO_SPEC;
impl crate::RegisterSpec for TRACE_CAPTURE_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_capture_fifo::R`](R) reader structure"]
impl crate::Readable for TRACE_CAPTURE_FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_capture_fifo::W`](W) writer structure"]
impl crate::Writable for TRACE_CAPTURE_FIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACE_CAPTURE_FIFO to value 0"]
impl crate::Resettable for TRACE_CAPTURE_FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
