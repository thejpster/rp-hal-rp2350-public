#[doc = "Register `HALTSUM1` reader"]
pub type R = crate::R<HALTSUM1_SPEC>;
#[doc = "Field `HALTSUM1` reader - "]
pub type HALTSUM1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn haltsum1(&self) -> HALTSUM1_R {
        HALTSUM1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Each bit in this read-only register indicates whether any of a group of harts is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 Each bit in haltsum1 is an OR reduction of 32 bits' worth of haltsum0. On RP2350, only the LSB is implemented.  

You can [`read`](crate::Reg::read) this register and get [`haltsum1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALTSUM1_SPEC;
impl crate::RegisterSpec for HALTSUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haltsum1::R`](R) reader structure"]
impl crate::Readable for HALTSUM1_SPEC {}
#[doc = "`reset()` method sets HALTSUM1 to value 0"]
impl crate::Resettable for HALTSUM1_SPEC {
    const RESET_VALUE: u32 = 0;
}
