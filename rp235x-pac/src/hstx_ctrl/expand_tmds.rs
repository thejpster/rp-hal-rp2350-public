#[doc = "Register `EXPAND_TMDS` reader"]
pub type R = crate::R<EXPAND_TMDS_SPEC>;
#[doc = "Register `EXPAND_TMDS` writer"]
pub type W = crate::W<EXPAND_TMDS_SPEC>;
#[doc = "Field `L0_ROT` reader - Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
pub type L0_ROT_R = crate::FieldReader;
#[doc = "Field `L0_ROT` writer - Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
pub type L0_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L0_NBITS` reader - Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L0_NBITS_R = crate::FieldReader;
#[doc = "Field `L0_NBITS` writer - Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L0_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `L1_ROT` reader - Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
pub type L1_ROT_R = crate::FieldReader;
#[doc = "Field `L1_ROT` writer - Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
pub type L1_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L1_NBITS` reader - Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L1_NBITS_R = crate::FieldReader;
#[doc = "Field `L1_NBITS` writer - Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L1_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `L2_ROT` reader - Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
pub type L2_ROT_R = crate::FieldReader;
#[doc = "Field `L2_ROT` writer - Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
pub type L2_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L2_NBITS` reader - Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L2_NBITS_R = crate::FieldReader;
#[doc = "Field `L2_NBITS` writer - Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
pub type L2_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
    #[inline(always)]
    pub fn l0_rot(&self) -> L0_ROT_R {
        L0_ROT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn l0_nbits(&self) -> L0_NBITS_R {
        L0_NBITS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
    #[inline(always)]
    pub fn l1_rot(&self) -> L1_ROT_R {
        L1_ROT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn l1_nbits(&self) -> L1_NBITS_R {
        L1_NBITS_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
    #[inline(always)]
    pub fn l2_rot(&self) -> L2_ROT_R {
        L2_ROT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn l2_nbits(&self) -> L2_NBITS_R {
        L2_NBITS_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
    #[inline(always)]
    #[must_use]
    pub fn l0_rot(&mut self) -> L0_ROT_W<EXPAND_TMDS_SPEC> {
        L0_ROT_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn l0_nbits(&mut self) -> L0_NBITS_W<EXPAND_TMDS_SPEC> {
        L0_NBITS_W::new(self, 5)
    }
    #[doc = "Bits 8:12 - Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
    #[inline(always)]
    #[must_use]
    pub fn l1_rot(&mut self) -> L1_ROT_W<EXPAND_TMDS_SPEC> {
        L1_ROT_W::new(self, 8)
    }
    #[doc = "Bits 13:15 - Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn l1_nbits(&mut self) -> L1_NBITS_W<EXPAND_TMDS_SPEC> {
        L1_NBITS_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
    #[inline(always)]
    #[must_use]
    pub fn l2_rot(&mut self) -> L2_ROT_W<EXPAND_TMDS_SPEC> {
        L2_ROT_W::new(self, 16)
    }
    #[doc = "Bits 21:23 - Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn l2_nbits(&mut self) -> L2_NBITS_W<EXPAND_TMDS_SPEC> {
        L2_NBITS_W::new(self, 21)
    }
}
#[doc = "Configure the optional TMDS encoder inside the command expander  

You can [`read`](crate::Reg::read) this register and get [`expand_tmds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_tmds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXPAND_TMDS_SPEC;
impl crate::RegisterSpec for EXPAND_TMDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`expand_tmds::R`](R) reader structure"]
impl crate::Readable for EXPAND_TMDS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`expand_tmds::W`](W) writer structure"]
impl crate::Writable for EXPAND_TMDS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXPAND_TMDS to value 0"]
impl crate::Resettable for EXPAND_TMDS_SPEC {
    const RESET_VALUE: u32 = 0;
}
