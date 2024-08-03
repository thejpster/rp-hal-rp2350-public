#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `VREG_OUTPUT_LOW` reader - "]
pub type VREG_OUTPUT_LOW_R = crate::BitReader;
#[doc = "Field `VREG_OUTPUT_LOW` writer - "]
pub type VREG_OUTPUT_LOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `STATE_REQ_IGNORED` reader - Source is state.req_ignored"]
pub type STATE_REQ_IGNORED_R = crate::BitReader;
#[doc = "Field `PWRUP_WHILE_WAITING` reader - Source is state.pwrup_while_waiting"]
pub type PWRUP_WHILE_WAITING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vreg_output_low(&self) -> VREG_OUTPUT_LOW_R {
        VREG_OUTPUT_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source is state.req_ignored"]
    #[inline(always)]
    pub fn state_req_ignored(&self) -> STATE_REQ_IGNORED_R {
        STATE_REQ_IGNORED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> PWRUP_WHILE_WAITING_R {
        PWRUP_WHILE_WAITING_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_output_low(&mut self) -> VREG_OUTPUT_LOW_W<INTR_SPEC> {
        VREG_OUTPUT_LOW_W::new(self, 0)
    }
}
#[doc = "Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
