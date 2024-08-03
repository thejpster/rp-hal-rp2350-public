#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `LEVEL` reader - "]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `FULL` reader - "]
pub type FULL_R = crate::BitReader;
#[doc = "Field `EMPTY` reader - "]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `WOF` reader - FIFO was written when full. Write 1 to clear."]
pub type WOF_R = crate::BitReader;
#[doc = "Field `WOF` writer - FIFO was written when full. Write 1 to clear."]
pub type WOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO was written when full. Write 1 to clear."]
    #[inline(always)]
    pub fn wof(&self) -> WOF_R {
        WOF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - FIFO was written when full. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn wof(&mut self) -> WOF_W<STAT_SPEC> {
        WOF_W::new(self, 10)
    }
}
#[doc = "FIFO status  

You can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0400;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
