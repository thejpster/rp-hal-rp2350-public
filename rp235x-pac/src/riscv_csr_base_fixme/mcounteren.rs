#[doc = "Register `MCOUNTEREN` reader"]
pub type R = crate::R<MCOUNTEREN_SPEC>;
#[doc = "Register `MCOUNTEREN` writer"]
pub type W = crate::W<MCOUNTEREN_SPEC>;
#[doc = "Field `CY` reader - If 1, U-mode is permitted to access the `cycle`/`cycleh` cycle counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
pub type CY_R = crate::BitReader;
#[doc = "Field `CY` writer - If 1, U-mode is permitted to access the `cycle`/`cycleh` cycle counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
pub type CY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - No hardware effect, as the `time`/`timeh` CSRs are not implemented. However, this field still exists, as M-mode software can use it to track whether it should emulate U-mode attempts to access those CSRs."]
pub type TM_R = crate::BitReader;
#[doc = "Field `TM` writer - No hardware effect, as the `time`/`timeh` CSRs are not implemented. However, this field still exists, as M-mode software can use it to track whether it should emulate U-mode attempts to access those CSRs."]
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR` reader - If 1, U-mode is permitted to access the `instret`/`instreth` instruction retire counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
pub type IR_R = crate::BitReader;
#[doc = "Field `IR` writer - If 1, U-mode is permitted to access the `instret`/`instreth` instruction retire counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
pub type IR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, U-mode is permitted to access the `cycle`/`cycleh` cycle counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
    #[inline(always)]
    pub fn cy(&self) -> CY_R {
        CY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No hardware effect, as the `time`/`timeh` CSRs are not implemented. However, this field still exists, as M-mode software can use it to track whether it should emulate U-mode attempts to access those CSRs."]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, U-mode is permitted to access the `instret`/`instreth` instruction retire counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, U-mode is permitted to access the `cycle`/`cycleh` cycle counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
    #[inline(always)]
    #[must_use]
    pub fn cy(&mut self) -> CY_W<MCOUNTEREN_SPEC> {
        CY_W::new(self, 0)
    }
    #[doc = "Bit 1 - No hardware effect, as the `time`/`timeh` CSRs are not implemented. However, this field still exists, as M-mode software can use it to track whether it should emulate U-mode attempts to access those CSRs."]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<MCOUNTEREN_SPEC> {
        TM_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, U-mode is permitted to access the `instret`/`instreth` instruction retire counter CSRs. Otherwise, U-mode accesses to these CSRs will trap."]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IR_W<MCOUNTEREN_SPEC> {
        IR_W::new(self, 2)
    }
}
#[doc = "Counter enable. Control access to counters from U-mode. Not to be confused with mcountinhibit.  

You can [`read`](crate::Reg::read) this register and get [`mcounteren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcounteren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCOUNTEREN_SPEC;
impl crate::RegisterSpec for MCOUNTEREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcounteren::R`](R) reader structure"]
impl crate::Readable for MCOUNTEREN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcounteren::W`](W) writer structure"]
impl crate::Writable for MCOUNTEREN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCOUNTEREN to value 0"]
impl crate::Resettable for MCOUNTEREN_SPEC {
    const RESET_VALUE: u32 = 0;
}
