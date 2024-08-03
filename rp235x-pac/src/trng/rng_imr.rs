#[doc = "Register `RNG_IMR` reader"]
pub type R = crate::R<RNG_IMR_SPEC>;
#[doc = "Register `RNG_IMR` writer"]
pub type W = crate::W<RNG_IMR_SPEC>;
#[doc = "Field `EHR_VALID_INT_MASK` reader - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type EHR_VALID_INT_MASK_R = crate::BitReader;
#[doc = "Field `EHR_VALID_INT_MASK` writer - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type EHR_VALID_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORR_ERR_INT_MASK` reader - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type AUTOCORR_ERR_INT_MASK_R = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR_INT_MASK` writer - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type AUTOCORR_ERR_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_ERR_INT_MASK` reader - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type CRNGT_ERR_INT_MASK_R = crate::BitReader;
#[doc = "Field `CRNGT_ERR_INT_MASK` writer - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type CRNGT_ERR_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VN_ERR_INT_MASK` reader - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type VN_ERR_INT_MASK_R = crate::BitReader;
#[doc = "Field `VN_ERR_INT_MASK` writer - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
pub type VN_ERR_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn ehr_valid_int_mask(&self) -> EHR_VALID_INT_MASK_R {
        EHR_VALID_INT_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn autocorr_err_int_mask(&self) -> AUTOCORR_ERR_INT_MASK_R {
        AUTOCORR_ERR_INT_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn crngt_err_int_mask(&self) -> CRNGT_ERR_INT_MASK_R {
        CRNGT_ERR_INT_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn vn_err_int_mask(&self) -> VN_ERR_INT_MASK_R {
        VN_ERR_INT_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ehr_valid_int_mask(&mut self) -> EHR_VALID_INT_MASK_W<RNG_IMR_SPEC> {
        EHR_VALID_INT_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_err_int_mask(&mut self) -> AUTOCORR_ERR_INT_MASK_W<RNG_IMR_SPEC> {
        AUTOCORR_ERR_INT_MASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crngt_err_int_mask(&mut self) -> CRNGT_ERR_INT_MASK_W<RNG_IMR_SPEC> {
        CRNGT_ERR_INT_MASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn vn_err_int_mask(&mut self) -> VN_ERR_INT_MASK_W<RNG_IMR_SPEC> {
        VN_ERR_INT_MASK_W::new(self, 3)
    }
}
#[doc = "Interrupt masking.  

You can [`read`](crate::Reg::read) this register and get [`rng_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_IMR_SPEC;
impl crate::RegisterSpec for RNG_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_imr::R`](R) reader structure"]
impl crate::Readable for RNG_IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_imr::W`](W) writer structure"]
impl crate::Writable for RNG_IMR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_IMR to value 0x0f"]
impl crate::Resettable for RNG_IMR_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
