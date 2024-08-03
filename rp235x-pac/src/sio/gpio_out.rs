#[doc = "Register `GPIO_OUT` reader"]
pub type R = crate::R<GPIO_OUT_SPEC>;
#[doc = "Register `GPIO_OUT` writer"]
pub type W = crate::W<GPIO_OUT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GPIO0...31 output value  
 Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.  

 If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.  

 In the NonSecure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.  

You can [`read`](crate::Reg::read) this register and get [`gpio_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_out::R`](R) reader structure"]
impl crate::Readable for GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_out::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OUT to value 0"]
impl crate::Resettable for GPIO_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
