#[doc = "Register `FIFO_RD` reader"]
pub type R = crate::R<FIFO_RD_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<FIFO_RD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Read access to this core's RX FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct FIFO_RD_SPEC;
impl crate::RegisterSpec for FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_rd::R`](R) reader structure"]
impl crate::Readable for FIFO_RD_SPEC {}
#[doc = "`reset()` method sets FIFO_RD to value 0"]
impl crate::Resettable for FIFO_RD_SPEC {
    const RESET_VALUE: u32 = 0;
}
