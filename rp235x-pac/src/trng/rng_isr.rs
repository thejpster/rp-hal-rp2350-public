#[doc = "Register `RNG_ISR` reader"]
pub type R = crate::R<RNG_ISR_SPEC>;
#[doc = "Register `RNG_ISR` writer"]
pub type W = crate::W<RNG_ISR_SPEC>;
#[doc = "Field `EHR_VALID` reader - 1'b1 indicates that 192 bits have been collected in the RNG, and are ready to be read."]
pub type EHR_VALID_R = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR` reader - 1'b1 indicates Autocorrelation test failed four times in a row. When set, RNG cease from functioning until next reset."]
pub type AUTOCORR_ERR_R = crate::BitReader;
#[doc = "Field `CRNGT_ERR` reader - 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
pub type CRNGT_ERR_R = crate::BitReader;
#[doc = "Field `VN_ERR` reader - 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecutive collected bits are identical, ZERO or ONE."]
pub type VN_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1'b1 indicates that 192 bits have been collected in the RNG, and are ready to be read."]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b1 indicates Autocorrelation test failed four times in a row. When set, RNG cease from functioning until next reset."]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AUTOCORR_ERR_R {
        AUTOCORR_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    pub fn crngt_err(&self) -> CRNGT_ERR_R {
        CRNGT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecutive collected bits are identical, ZERO or ONE."]
    #[inline(always)]
    pub fn vn_err(&self) -> VN_ERR_R {
        VN_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated.  

You can [`read`](crate::Reg::read) this register and get [`rng_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_ISR_SPEC;
impl crate::RegisterSpec for RNG_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_isr::R`](R) reader structure"]
impl crate::Readable for RNG_ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_isr::W`](W) writer structure"]
impl crate::Writable for RNG_ISR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_ISR to value 0"]
impl crate::Resettable for RNG_ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
