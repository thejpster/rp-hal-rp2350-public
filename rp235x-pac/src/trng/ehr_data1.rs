#[doc = "Register `EHR_DATA1` reader"]
pub type R = crate::R<EHR_DATA1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "RNG collected bits.  
 Bits \\[63:32\\]
of Entropy Holding Register (EHR) - RNG output register  

You can [`read`](crate::Reg::read) this register and get [`ehr_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA1_SPEC;
impl crate::RegisterSpec for EHR_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data1::R`](R) reader structure"]
impl crate::Readable for EHR_DATA1_SPEC {}
#[doc = "`reset()` method sets EHR_DATA1 to value 0"]
impl crate::Resettable for EHR_DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
