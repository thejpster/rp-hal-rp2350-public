#[doc = "Register `TRCSYNCPR` reader"]
pub type R = crate::R<TRCSYNCPR_SPEC>;
#[doc = "Field `PERIOD` reader - Defines the number of bytes of trace between trace synchronization requests as a total of the number of bytes generated by the instruction stream. The number of bytes is 2N where N is the value of this field: - A value of zero disables these periodic trace synchronization requests, but does not disable other trace synchronization requests. - The minimum value that can be programmed, other than zero, is 8, providing a minimum trace synchronization period of 256 bytes. - The maximum value is 20, providing a maximum trace synchronization period of 2^20 bytes"]
pub type PERIOD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Defines the number of bytes of trace between trace synchronization requests as a total of the number of bytes generated by the instruction stream. The number of bytes is 2N where N is the value of this field: - A value of zero disables these periodic trace synchronization requests, but does not disable other trace synchronization requests. - The minimum value that can be programmed, other than zero, is 8, providing a minimum trace synchronization period of 256 bytes. - The maximum value is 20, providing a maximum trace synchronization period of 2^20 bytes"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "The TRCSYNCPR specifies the period of trace synchronization of the trace streams. TRCSYNCPR defines a number of bytes of trace between requests for trace synchronization. This value is always a power of two  

You can [`read`](crate::Reg::read) this register and get [`trcsyncpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCSYNCPR_SPEC;
impl crate::RegisterSpec for TRCSYNCPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsyncpr::R`](R) reader structure"]
impl crate::Readable for TRCSYNCPR_SPEC {}
#[doc = "`reset()` method sets TRCSYNCPR to value 0x0a"]
impl crate::Resettable for TRCSYNCPR_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
