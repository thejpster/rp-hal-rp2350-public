#[doc = "Register `HALTSUM0` reader"]
pub type R = crate::R<HALTSUM0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Each bit in this read-only register indicates whether one specific hart is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 On RP2350, only the two LSBs of this register are implemented, one for each core/hart.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`haltsum0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALTSUM0_SPEC;
impl crate::RegisterSpec for HALTSUM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum0::R`](R) reader structure"]
impl crate::Readable for HALTSUM0_SPEC {}
#[doc = "`reset()` method sets HALTSUM0 to value 0"]
impl crate::Resettable for HALTSUM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
