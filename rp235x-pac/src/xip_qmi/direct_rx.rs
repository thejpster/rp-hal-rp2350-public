#[doc = "Register `DIRECT_RX` reader"]
pub type R = crate::R<DIRECT_RX_SPEC>;
#[doc = "Field `DIRECT_RX` reader - With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.  

 When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.  

<div class=\"warning\">One or more dependent resources other than the current field are immediately affected by a read operation.</div>"]
pub type DIRECT_RX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.  

 When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received."]
    #[inline(always)]
    pub fn direct_rx(&self) -> DIRECT_RX_R {
        DIRECT_RX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO for direct mode  

You can [`read`](crate::Reg::read) this register and get [`direct_rx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRECT_RX_SPEC;
impl crate::RegisterSpec for DIRECT_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direct_rx::R`](R) reader structure"]
impl crate::Readable for DIRECT_RX_SPEC {}
#[doc = "`reset()` method sets DIRECT_RX to value 0"]
impl crate::Resettable for DIRECT_RX_SPEC {
    const RESET_VALUE: u32 = 0;
}
