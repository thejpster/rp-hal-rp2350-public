#[doc = "Register `CURRENT_PWRUP_REQ` reader"]
pub type R = crate::R<CURRENT_PWRUP_REQ_SPEC>;
#[doc = "Field `CURRENT_PWRUP_REQ` reader - "]
pub type CURRENT_PWRUP_REQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn current_pwrup_req(&self) -> CURRENT_PWRUP_REQ_R {
        CURRENT_PWRUP_REQ_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Indicates current powerup request state  
 pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab  
 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET  
 1 = pwrup0  
 2 = pwrup1  
 3 = pwrup2  
 4 = pwrup3  
 5 = coresight_pwrup  
 6 = alarm_pwrup  

You can [`read`](crate::Reg::read) this register and get [`current_pwrup_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_PWRUP_REQ_SPEC;
impl crate::RegisterSpec for CURRENT_PWRUP_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_pwrup_req::R`](R) reader structure"]
impl crate::Readable for CURRENT_PWRUP_REQ_SPEC {}
#[doc = "`reset()` method sets CURRENT_PWRUP_REQ to value 0"]
impl crate::Resettable for CURRENT_PWRUP_REQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
