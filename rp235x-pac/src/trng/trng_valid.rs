#[doc = "Register `TRNG_VALID` reader"]
pub type R = crate::R<TRNG_VALID_SPEC>;
#[doc = "Field `EHR_VALID` reader - 1'b1 indicates that collection of bits in the RNG is completed, and data can be read from EHR_DATA register."]
pub type EHR_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1'b1 indicates that collection of bits in the RNG is completed, and data can be read from EHR_DATA register."]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 1) != 0)
    }
}
#[doc = "192 bit collection indication.  

You can [`read`](crate::Reg::read) this register and get [`trng_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRNG_VALID_SPEC;
impl crate::RegisterSpec for TRNG_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_valid::R`](R) reader structure"]
impl crate::Readable for TRNG_VALID_SPEC {}
#[doc = "`reset()` method sets TRNG_VALID to value 0"]
impl crate::Resettable for TRNG_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
