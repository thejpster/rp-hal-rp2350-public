#[doc = "Register `RNG_ICR` reader"]
pub type R = crate::R<RNG_ICR_SPEC>;
#[doc = "Register `RNG_ICR` writer"]
pub type W = crate::W<RNG_ICR_SPEC>;
#[doc = "Field `EHR_VALID` reader - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type EHR_VALID_R = crate::BitReader;
#[doc = "Field `EHR_VALID` writer - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type EHR_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORR_ERR` reader - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type AUTOCORR_ERR_R = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR` writer - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type AUTOCORR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_ERR` reader - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type CRNGT_ERR_R = crate::BitReader;
#[doc = "Field `CRNGT_ERR` writer - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type CRNGT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VN_ERR` reader - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type VN_ERR_R = crate::BitReader;
#[doc = "Field `VN_ERR` writer - Write 1'b1 - clear corresponding bit in RNG_ISR."]
pub type VN_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AUTOCORR_ERR_R {
        AUTOCORR_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn crngt_err(&self) -> CRNGT_ERR_R {
        CRNGT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn vn_err(&self) -> VN_ERR_R {
        VN_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    #[must_use]
    pub fn ehr_valid(&mut self) -> EHR_VALID_W<RNG_ICR_SPEC> {
        EHR_VALID_W::new(self, 0)
    }
    #[doc = "Bit 1 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_err(&mut self) -> AUTOCORR_ERR_W<RNG_ICR_SPEC> {
        AUTOCORR_ERR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    #[must_use]
    pub fn crngt_err(&mut self) -> CRNGT_ERR_W<RNG_ICR_SPEC> {
        CRNGT_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    #[must_use]
    pub fn vn_err(&mut self) -> VN_ERR_W<RNG_ICR_SPEC> {
        VN_ERR_W::new(self, 3)
    }
}
#[doc = "Interrupt/status bit clear Register.  

You can [`read`](crate::Reg::read) this register and get [`rng_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_ICR_SPEC;
impl crate::RegisterSpec for RNG_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_icr::R`](R) reader structure"]
impl crate::Readable for RNG_ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_icr::W`](W) writer structure"]
impl crate::Writable for RNG_ICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_ICR to value 0"]
impl crate::Resettable for RNG_ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
