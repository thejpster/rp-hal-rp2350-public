#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `RES1_1` reader - Reserved, RES1"]
pub type RES1_1_R = crate::BitReader;
#[doc = "Field `USERSETMPEND` reader - Determines whether unprivileged accesses are permitted to pend interrupts via the STIR"]
pub type USERSETMPEND_R = crate::BitReader;
#[doc = "Field `USERSETMPEND` writer - Determines whether unprivileged accesses are permitted to pend interrupts via the STIR"]
pub type USERSETMPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNALIGN_TRP` reader - Controls the trapping of unaligned word or halfword accesses"]
pub type UNALIGN_TRP_R = crate::BitReader;
#[doc = "Field `UNALIGN_TRP` writer - Controls the trapping of unaligned word or halfword accesses"]
pub type UNALIGN_TRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_0_TRP` reader - Controls the generation of a DIVBYZERO UsageFault when attempting to perform integer division by zero"]
pub type DIV_0_TRP_R = crate::BitReader;
#[doc = "Field `DIV_0_TRP` writer - Controls the generation of a DIVBYZERO UsageFault when attempting to perform integer division by zero"]
pub type DIV_0_TRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFHFNMIGN` reader - Determines the effect of precise BusFaults on handlers running at a requested priority less than 0"]
pub type BFHFNMIGN_R = crate::BitReader;
#[doc = "Field `BFHFNMIGN` writer - Determines the effect of precise BusFaults on handlers running at a requested priority less than 0"]
pub type BFHFNMIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES1` reader - Reserved, RES1"]
pub type RES1_R = crate::BitReader;
#[doc = "Field `STKOFHFNMIGN` reader - Controls the effect of a stack limit violation while executing at a requested priority less than 0"]
pub type STKOFHFNMIGN_R = crate::BitReader;
#[doc = "Field `STKOFHFNMIGN` writer - Controls the effect of a stack limit violation while executing at a requested priority less than 0"]
pub type STKOFHFNMIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Enables data caching of all data accesses to Normal memory `FTSSS"]
pub type DC_R = crate::BitReader;
#[doc = "Field `IC` reader - This is a global enable bit for instruction caches in the selected Security state"]
pub type IC_R = crate::BitReader;
#[doc = "Field `BP` reader - Enables program flow prediction `FTSSS"]
pub type BP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1_1(&self) -> RES1_1_R {
        RES1_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines whether unprivileged accesses are permitted to pend interrupts via the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the trapping of unaligned word or halfword accesses"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the generation of a DIVBYZERO UsageFault when attempting to perform integer division by zero"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Determines the effect of precise BusFaults on handlers running at a requested priority less than 0"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1(&self) -> RES1_R {
        RES1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls the effect of a stack limit violation while executing at a requested priority less than 0"]
    #[inline(always)]
    pub fn stkofhfnmign(&self) -> STKOFHFNMIGN_R {
        STKOFHFNMIGN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data caching of all data accesses to Normal memory `FTSSS"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is a global enable bit for instruction caches in the selected Security state"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables program flow prediction `FTSSS"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Determines whether unprivileged accesses are permitted to pend interrupts via the STIR"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W<CCR_SPEC> {
        USERSETMPEND_W::new(self, 1)
    }
    #[doc = "Bit 3 - Controls the trapping of unaligned word or halfword accesses"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W<CCR_SPEC> {
        UNALIGN_TRP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Controls the generation of a DIVBYZERO UsageFault when attempting to perform integer division by zero"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W<CCR_SPEC> {
        DIV_0_TRP_W::new(self, 4)
    }
    #[doc = "Bit 8 - Determines the effect of precise BusFaults on handlers running at a requested priority less than 0"]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W<CCR_SPEC> {
        BFHFNMIGN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Controls the effect of a stack limit violation while executing at a requested priority less than 0"]
    #[inline(always)]
    #[must_use]
    pub fn stkofhfnmign(&mut self) -> STKOFHFNMIGN_W<CCR_SPEC> {
        STKOFHFNMIGN_W::new(self, 10)
    }
}
#[doc = "Sets or returns configuration and control data  

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0201"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0x0201;
}
