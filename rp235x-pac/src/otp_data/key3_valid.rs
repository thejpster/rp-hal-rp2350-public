#[doc = "Register `KEY3_VALID` reader"]
pub type R = crate::R<KEY3_VALID_SPEC>;
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
#[doc = "Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key3_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY3_VALID_SPEC;
impl crate::RegisterSpec for KEY3_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_valid::R`](R) reader structure"]
impl crate::Readable for KEY3_VALID_SPEC {}
#[doc = "`reset()` method sets KEY3_VALID to value 0"]
impl crate::Resettable for KEY3_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
