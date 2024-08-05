#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Register `INTS` writer"]
pub type W = crate::W<INTS_SPEC>;
#[doc = "Field `VREG_OUTPUT_LOW` reader - "]
pub type VREG_OUTPUT_LOW_R = crate::BitReader;
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
impl W {}
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints::W`](W) writer structure"]
impl crate::Writable for INTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
