#[doc = "Register `TRCSTALLCTLR` reader"]
pub type R = crate::R<TRCSTALLCTLR_SPEC>;
#[doc = "Register `TRCSTALLCTLR` writer"]
pub type W = crate::W<TRCSTALLCTLR_SPEC>;
#[doc = "Field `LEVEL` reader - Threshold at which stalling becomes active. This provides four levels. This level can be varied to optimize the level of invasion caused by stalling, balanced against the risk of a FIFO overflow"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold at which stalling becomes active. This provides four levels. This level can be varied to optimize the level of invasion caused by stalling, balanced against the risk of a FIFO overflow"]
pub type LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ISTALL` reader - Stall processor based on instruction trace buffer space"]
pub type ISTALL_R = crate::BitReader;
#[doc = "Field `ISTALL` writer - Stall processor based on instruction trace buffer space"]
pub type ISTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTPRIORITY` reader - Reserved, RES0"]
pub type INSTPRIORITY_R = crate::BitReader;
impl R {
    #[doc = "Bits 2:3 - Threshold at which stalling becomes active. This provides four levels. This level can be varied to optimize the level of invasion caused by stalling, balanced against the risk of a FIFO overflow"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - Stall processor based on instruction trace buffer space"]
    #[inline(always)]
    pub fn istall(&self) -> ISTALL_R {
        ISTALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved, RES0"]
    #[inline(always)]
    pub fn instpriority(&self) -> INSTPRIORITY_R {
        INSTPRIORITY_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Threshold at which stalling becomes active. This provides four levels. This level can be varied to optimize the level of invasion caused by stalling, balanced against the risk of a FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<TRCSTALLCTLR_SPEC> {
        LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 8 - Stall processor based on instruction trace buffer space"]
    #[inline(always)]
    #[must_use]
    pub fn istall(&mut self) -> ISTALL_W<TRCSTALLCTLR_SPEC> {
        ISTALL_W::new(self, 8)
    }
}
#[doc = "The TRCSTALLCTLR enables ETM-Teal to stall the processor if the ETM-Teal FIFO goes over the programmed level to minimize risk of overflow  

You can [`read`](crate::Reg::read) this register and get [`trcstallctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstallctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCSTALLCTLR_SPEC;
impl crate::RegisterSpec for TRCSTALLCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcstallctlr::R`](R) reader structure"]
impl crate::Readable for TRCSTALLCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcstallctlr::W`](W) writer structure"]
impl crate::Writable for TRCSTALLCTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCSTALLCTLR to value 0"]
impl crate::Resettable for TRCSTALLCTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
