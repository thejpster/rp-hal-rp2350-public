#[doc = "Register `GPIO_IN` reader"]
pub type R = crate::R<GPIO_IN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Input value for GPIO0...31.  

 In the NonSecure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.  

You can [`read`](crate::Reg::read) this register and get [`gpio_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_in::R`](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
