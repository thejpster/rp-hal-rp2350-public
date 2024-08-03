#[doc = "Register `ALARM1` reader"]
pub type R = crate::R<ALARM1_SPEC>;
#[doc = "Register `ALARM1` writer"]
pub type W = crate::W<ALARM1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Arm alarm 1, and configure the time it will fire.  
 Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.  
 The alarm will disarm itself once it fires, and can  
 be disarmed early using the ARMED status register.  

You can [`read`](crate::Reg::read) this register and get [`alarm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM1_SPEC;
impl crate::RegisterSpec for ALARM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm1::R`](R) reader structure"]
impl crate::Readable for ALARM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm1::W`](W) writer structure"]
impl crate::Writable for ALARM1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM1 to value 0"]
impl crate::Resettable for ALARM1_SPEC {
    const RESET_VALUE: u32 = 0;
}