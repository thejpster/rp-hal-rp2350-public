#[doc = "Register `INTE` reader"]
pub type R = crate::R<INTE_SPEC>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<INTE_SPEC>;
#[doc = "Field `VREG_OUTPUT_LOW` reader - "]
pub type VREG_OUTPUT_LOW_R = crate::BitReader;
#[doc = "Field `VREG_OUTPUT_LOW` writer - "]
pub type VREG_OUTPUT_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER` reader - "]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `TIMER` writer - "]
pub type TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATE_REQ_IGNORED` reader - Source is state.req_ignored"]
pub type STATE_REQ_IGNORED_R = crate::BitReader;
#[doc = "Field `STATE_REQ_IGNORED` writer - Source is state.req_ignored"]
pub type STATE_REQ_IGNORED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUP_WHILE_WAITING` reader - Source is state.pwrup_while_waiting"]
pub type PWRUP_WHILE_WAITING_R = crate::BitReader;
#[doc = "Field `PWRUP_WHILE_WAITING` writer - Source is state.pwrup_while_waiting"]
pub type PWRUP_WHILE_WAITING_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn vreg_output_low(&mut self) -> VREG_OUTPUT_LOW_W<INTE_SPEC> {
        VREG_OUTPUT_LOW_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<INTE_SPEC> {
        TIMER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Source is state.req_ignored"]
    #[inline(always)]
    #[must_use]
    pub fn state_req_ignored(&mut self) -> STATE_REQ_IGNORED_W<INTE_SPEC> {
        STATE_REQ_IGNORED_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source is state.pwrup_while_waiting"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_while_waiting(&mut self) -> PWRUP_WHILE_WAITING_W<INTE_SPEC> {
        PWRUP_WHILE_WAITING_W::new(self, 3)
    }
}
#[doc = "Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
