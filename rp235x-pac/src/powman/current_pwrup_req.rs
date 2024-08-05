#[doc = "Register `CURRENT_PWRUP_REQ` reader"]
pub type R = crate::R<CURRENT_PWRUP_REQ_SPEC>;
#[doc = "Register `CURRENT_PWRUP_REQ` writer"]
pub type W = crate::W<CURRENT_PWRUP_REQ_SPEC>;
#[doc = "Field `CURRENT_PWRUP_REQ` reader - "]
pub type CURRENT_PWRUP_REQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn current_pwrup_req(&self) -> CURRENT_PWRUP_REQ_R {
        CURRENT_PWRUP_REQ_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup  

You can [`read`](crate::Reg::read) this register and get [`current_pwrup_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current_pwrup_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_PWRUP_REQ_SPEC;
impl crate::RegisterSpec for CURRENT_PWRUP_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_pwrup_req::R`](R) reader structure"]
impl crate::Readable for CURRENT_PWRUP_REQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`current_pwrup_req::W`](W) writer structure"]
impl crate::Writable for CURRENT_PWRUP_REQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURRENT_PWRUP_REQ to value 0"]
impl crate::Resettable for CURRENT_PWRUP_REQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
