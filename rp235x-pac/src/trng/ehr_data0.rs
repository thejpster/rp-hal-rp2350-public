#[doc = "Register `EHR_DATA0` reader"]
pub type R = crate::R<EHR_DATA0_SPEC>;
#[doc = "Register `EHR_DATA0` writer"]
pub type W = crate::W<EHR_DATA0_SPEC>;
#[doc = "Field `EHR_DATA0` reader - Bits \\[31:0\\]
of Entropy Holding Register (EHR) - RNG output register"]
pub type EHR_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of Entropy Holding Register (EHR) - RNG output register"]
    #[inline(always)]
    pub fn ehr_data0(&self) -> EHR_DATA0_R {
        EHR_DATA0_R::new(self.bits)
    }
}
impl W {}
#[doc = "RNG collected bits.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehr_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA0_SPEC;
impl crate::RegisterSpec for EHR_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data0::R`](R) reader structure"]
impl crate::Readable for EHR_DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ehr_data0::W`](W) writer structure"]
impl crate::Writable for EHR_DATA0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EHR_DATA0 to value 0"]
impl crate::Resettable for EHR_DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
