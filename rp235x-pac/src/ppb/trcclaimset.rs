#[doc = "Register `TRCCLAIMSET` reader"]
pub type R = crate::R<TRCCLAIMSET_SPEC>;
#[doc = "Register `TRCCLAIMSET` writer"]
pub type W = crate::W<TRCCLAIMSET_SPEC>;
#[doc = "Field `SET0` reader - When a write to one of these bits occurs, with the value:"]
pub type SET0_R = crate::BitReader;
#[doc = "Field `SET0` writer - When a write to one of these bits occurs, with the value:"]
pub type SET0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET1` reader - When a write to one of these bits occurs, with the value:"]
pub type SET1_R = crate::BitReader;
#[doc = "Field `SET1` writer - When a write to one of these bits occurs, with the value:"]
pub type SET1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET2` reader - When a write to one of these bits occurs, with the value:"]
pub type SET2_R = crate::BitReader;
#[doc = "Field `SET2` writer - When a write to one of these bits occurs, with the value:"]
pub type SET2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET3` reader - When a write to one of these bits occurs, with the value:"]
pub type SET3_R = crate::BitReader;
#[doc = "Field `SET3` writer - When a write to one of these bits occurs, with the value:"]
pub type SET3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn set0(&self) -> SET0_R {
        SET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn set1(&self) -> SET1_R {
        SET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn set2(&self) -> SET2_R {
        SET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn set3(&self) -> SET3_R {
        SET3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn set0(&mut self) -> SET0_W<TRCCLAIMSET_SPEC> {
        SET0_W::new(self, 0)
    }
    #[doc = "Bit 1 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn set1(&mut self) -> SET1_W<TRCCLAIMSET_SPEC> {
        SET1_W::new(self, 1)
    }
    #[doc = "Bit 2 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> SET2_W<TRCCLAIMSET_SPEC> {
        SET2_W::new(self, 2)
    }
    #[doc = "Bit 3 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> SET3_W<TRCCLAIMSET_SPEC> {
        SET3_W::new(self, 3)
    }
}
#[doc = "Claim Tag Set Register  

You can [`read`](crate::Reg::read) this register and get [`trcclaimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCLAIMSET_SPEC;
impl crate::RegisterSpec for TRCCLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclaimset::R`](R) reader structure"]
impl crate::Readable for TRCCLAIMSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcclaimset::W`](W) writer structure"]
impl crate::Writable for TRCCLAIMSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCLAIMSET to value 0x0f"]
impl crate::Resettable for TRCCLAIMSET_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
