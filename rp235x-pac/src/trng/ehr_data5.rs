#[doc = "Register `EHR_DATA5` reader"]
pub type R = crate::R<EHR_DATA5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "RNG collected bits.  
 Bits \\[191:160\\]
of Entropy Holding Register (EHR) - RNG output register  

You can [`read`](crate::Reg::read) this register and get [`ehr_data5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA5_SPEC;
impl crate::RegisterSpec for EHR_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data5::R`](R) reader structure"]
impl crate::Readable for EHR_DATA5_SPEC {}
#[doc = "`reset()` method sets EHR_DATA5 to value 0"]
impl crate::Resettable for EHR_DATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
