#[doc = "Register `TRCCLAIMCLR` reader"]
pub type R = crate::R<TRCCLAIMCLR_SPEC>;
#[doc = "Register `TRCCLAIMCLR` writer"]
pub type W = crate::W<TRCCLAIMCLR_SPEC>;
#[doc = "Field `CLR0` reader - When a write to one of these bits occurs, with the value:"]
pub type CLR0_R = crate::BitReader;
#[doc = "Field `CLR0` writer - When a write to one of these bits occurs, with the value:"]
pub type CLR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR1` reader - When a write to one of these bits occurs, with the value:"]
pub type CLR1_R = crate::BitReader;
#[doc = "Field `CLR1` writer - When a write to one of these bits occurs, with the value:"]
pub type CLR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR2` reader - When a write to one of these bits occurs, with the value:"]
pub type CLR2_R = crate::BitReader;
#[doc = "Field `CLR2` writer - When a write to one of these bits occurs, with the value:"]
pub type CLR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR3` reader - When a write to one of these bits occurs, with the value:"]
pub type CLR3_R = crate::BitReader;
#[doc = "Field `CLR3` writer - When a write to one of these bits occurs, with the value:"]
pub type CLR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn clr0(&self) -> CLR0_R {
        CLR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn clr1(&self) -> CLR1_R {
        CLR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn clr2(&self) -> CLR2_R {
        CLR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    pub fn clr3(&self) -> CLR3_R {
        CLR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> CLR0_W<TRCCLAIMCLR_SPEC> {
        CLR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> CLR1_W<TRCCLAIMCLR_SPEC> {
        CLR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> CLR2_W<TRCCLAIMCLR_SPEC> {
        CLR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - When a write to one of these bits occurs, with the value:"]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> CLR3_W<TRCCLAIMCLR_SPEC> {
        CLR3_W::new(self, 3)
    }
}
#[doc = "Claim Tag Clear Register  

You can [`read`](crate::Reg::read) this register and get [`trcclaimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCLAIMCLR_SPEC;
impl crate::RegisterSpec for TRCCLAIMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclaimclr::R`](R) reader structure"]
impl crate::Readable for TRCCLAIMCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcclaimclr::W`](W) writer structure"]
impl crate::Writable for TRCCLAIMCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCLAIMCLR to value 0"]
impl crate::Resettable for TRCCLAIMCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
