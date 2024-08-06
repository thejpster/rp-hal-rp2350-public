#[doc = "Register `RNG_VERSION` reader"]
pub type R = crate::R<RNG_VERSION_SPEC>;
#[doc = "Register `RNG_VERSION` writer"]
pub type W = crate::W<RNG_VERSION_SPEC>;
#[doc = "Field `EHR_WIDTH_192` reader - * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR"]
pub type EHR_WIDTH_192_R = crate::BitReader;
#[doc = "Field `CRNGT_EXISTS` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type CRNGT_EXISTS_R = crate::BitReader;
#[doc = "Field `AUTOCORR_EXISTS` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type AUTOCORR_EXISTS_R = crate::BitReader;
#[doc = "Field `TRNG_TESTS_BYPASS_EN` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type TRNG_TESTS_BYPASS_EN_R = crate::BitReader;
#[doc = "Field `PRNG_EXISTS` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type PRNG_EXISTS_R = crate::BitReader;
#[doc = "Field `KAT_EXISTS` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type KAT_EXISTS_R = crate::BitReader;
#[doc = "Field `RESEEDING_EXISTS` reader - * 1'b1 - Exists. *1'b0 - Does not exist"]
pub type RESEEDING_EXISTS_R = crate::BitReader;
#[doc = "Field `RNG_USE_5_SBOXES` reader - * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES"]
pub type RNG_USE_5_SBOXES_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - * 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR"]
    #[inline(always)]
    pub fn ehr_width_192(&self) -> EHR_WIDTH_192_R {
        EHR_WIDTH_192_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn crngt_exists(&self) -> CRNGT_EXISTS_R {
        CRNGT_EXISTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn autocorr_exists(&self) -> AUTOCORR_EXISTS_R {
        AUTOCORR_EXISTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn trng_tests_bypass_en(&self) -> TRNG_TESTS_BYPASS_EN_R {
        TRNG_TESTS_BYPASS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn prng_exists(&self) -> PRNG_EXISTS_R {
        PRNG_EXISTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn kat_exists(&self) -> KAT_EXISTS_R {
        KAT_EXISTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - * 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn reseeding_exists(&self) -> RESEEDING_EXISTS_R {
        RESEEDING_EXISTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - * 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES"]
    #[inline(always)]
    pub fn rng_use_5_sboxes(&self) -> RNG_USE_5_SBOXES_R {
        RNG_USE_5_SBOXES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "Displays the version settings of the TRNG.  

You can [`read`](crate::Reg::read) this register and get [`rng_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_VERSION_SPEC;
impl crate::RegisterSpec for RNG_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_version::R`](R) reader structure"]
impl crate::Readable for RNG_VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_version::W`](W) writer structure"]
impl crate::Writable for RNG_VERSION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_VERSION to value 0"]
impl crate::Resettable for RNG_VERSION_SPEC {
    const RESET_VALUE: u32 = 0;
}
