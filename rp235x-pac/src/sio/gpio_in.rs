#[doc = "Register `GPIO_IN` reader"]
pub type R = crate::R<GPIO_IN_SPEC>;
#[doc = "Register `GPIO_IN` writer"]
pub type W = crate::W<GPIO_IN_SPEC>;
#[doc = "Field `GPIO_IN` reader - "]
pub type GPIO_IN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_in(&self) -> GPIO_IN_R {
        GPIO_IN_R::new(self.bits)
    }
}
impl W {}
#[doc = "Input value for GPIO0...31. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.  

You can [`read`](crate::Reg::read) this register and get [`gpio_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_in::R`](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_in::W`](W) writer structure"]
impl crate::Writable for GPIO_IN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
