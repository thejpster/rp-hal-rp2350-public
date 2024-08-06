#[doc = "Register `EHR_DATA1` reader"]
pub type R = crate::R<EHR_DATA1_SPEC>;
#[doc = "Register `EHR_DATA1` writer"]
pub type W = crate::W<EHR_DATA1_SPEC>;
#[doc = "Field `EHR_DATA1` reader - Bits \\[63:32\\]
of Entropy Holding Register (EHR) - RNG output register"]
pub type EHR_DATA1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of Entropy Holding Register (EHR) - RNG output register"]
    #[inline(always)]
    pub fn ehr_data1(&self) -> EHR_DATA1_R {
        EHR_DATA1_R::new(self.bits)
    }
}
impl W {}
#[doc = "RNG collected bits.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehr_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA1_SPEC;
impl crate::RegisterSpec for EHR_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data1::R`](R) reader structure"]
impl crate::Readable for EHR_DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ehr_data1::W`](W) writer structure"]
impl crate::Writable for EHR_DATA1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EHR_DATA1 to value 0"]
impl crate::Resettable for EHR_DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
