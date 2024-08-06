#[doc = "Register `KEY6_VALID` reader"]
pub type R = crate::R<KEY6_VALID_SPEC>;
#[doc = "Register `KEY6_VALID` writer"]
pub type W = crate::W<KEY6_VALID_SPEC>;
#[doc = "Field `VALID` reader - "]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID_R1` reader - Redundant copy of VALID, with 3-way majority vote"]
pub type VALID_R1_R = crate::BitReader;
#[doc = "Field `VALID_R2` reader - Redundant copy of VALID, with 3-way majority vote"]
pub type VALID_R2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn valid_r1(&self) -> VALID_R1_R {
        VALID_R1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn valid_r2(&self) -> VALID_R2_R {
        VALID_R2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {}
#[doc = "Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key6_valid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_valid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY6_VALID_SPEC;
impl crate::RegisterSpec for KEY6_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6_valid::R`](R) reader structure"]
impl crate::Readable for KEY6_VALID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key6_valid::W`](W) writer structure"]
impl crate::Writable for KEY6_VALID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY6_VALID to value 0"]
impl crate::Resettable for KEY6_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
