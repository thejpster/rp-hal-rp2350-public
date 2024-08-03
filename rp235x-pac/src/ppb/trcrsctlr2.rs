#[doc = "Register `TRCRSCTLR2` reader"]
pub type R = crate::R<TRCRSCTLR2_SPEC>;
#[doc = "Register `TRCRSCTLR2` writer"]
pub type W = crate::W<TRCRSCTLR2_SPEC>;
#[doc = "Field `SELECT` reader - Selects one or more resources from the wanted group. One bit is provided per resource from the group"]
pub type SELECT_R = crate::FieldReader;
#[doc = "Field `SELECT` writer - Selects one or more resources from the wanted group. One bit is provided per resource from the group"]
pub type SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GROUP` reader - Selects a group of resource"]
pub type GROUP_R = crate::FieldReader;
#[doc = "Field `GROUP` writer - Selects a group of resource"]
pub type GROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INV` reader - Inverts the selected resources"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Inverts the selected resources"]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIRINV` reader - Inverts the result of a combined pair of resources. This bit is only implemented on the lower register for a pair of resource selectors"]
pub type PAIRINV_R = crate::BitReader;
#[doc = "Field `PAIRINV` writer - Inverts the result of a combined pair of resources. This bit is only implemented on the lower register for a pair of resource selectors"]
pub type PAIRINV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Selects one or more resources from the wanted group. One bit is provided per resource from the group"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Selects a group of resource"]
    #[inline(always)]
    pub fn group(&self) -> GROUP_R {
        GROUP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Inverts the selected resources"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Inverts the result of a combined pair of resources. This bit is only implemented on the lower register for a pair of resource selectors"]
    #[inline(always)]
    pub fn pairinv(&self) -> PAIRINV_R {
        PAIRINV_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects one or more resources from the wanted group. One bit is provided per resource from the group"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<TRCRSCTLR2_SPEC> {
        SELECT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - Selects a group of resource"]
    #[inline(always)]
    #[must_use]
    pub fn group(&mut self) -> GROUP_W<TRCRSCTLR2_SPEC> {
        GROUP_W::new(self, 16)
    }
    #[doc = "Bit 20 - Inverts the selected resources"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<TRCRSCTLR2_SPEC> {
        INV_W::new(self, 20)
    }
    #[doc = "Bit 21 - Inverts the result of a combined pair of resources. This bit is only implemented on the lower register for a pair of resource selectors"]
    #[inline(always)]
    #[must_use]
    pub fn pairinv(&mut self) -> PAIRINV_W<TRCRSCTLR2_SPEC> {
        PAIRINV_W::new(self, 21)
    }
}
#[doc = "The TRCRSCTLR controls the trace resources  

You can [`read`](crate::Reg::read) this register and get [`trcrsctlr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcrsctlr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCRSCTLR2_SPEC;
impl crate::RegisterSpec for TRCRSCTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcrsctlr2::R`](R) reader structure"]
impl crate::Readable for TRCRSCTLR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcrsctlr2::W`](W) writer structure"]
impl crate::Writable for TRCRSCTLR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCRSCTLR2 to value 0"]
impl crate::Resettable for TRCRSCTLR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
