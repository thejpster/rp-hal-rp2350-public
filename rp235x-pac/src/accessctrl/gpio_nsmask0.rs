#[doc = "Register `GPIO_NSMASK0` reader"]
pub type R = crate::R<GPIO_NSMASK0_SPEC>;
#[doc = "Register `GPIO_NSMASK0` writer"]
pub type W = crate::W<GPIO_NSMASK0_SPEC>;
#[doc = "Field `GPIO_NSMASK0` reader - "]
pub type GPIO_NSMASK0_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_NSMASK0` writer - "]
pub type GPIO_NSMASK0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_nsmask0(&self) -> GPIO_NSMASK0_R {
        GPIO_NSMASK0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_nsmask0(&mut self) -> GPIO_NSMASK0_W<GPIO_NSMASK0_SPEC> {
        GPIO_NSMASK0_W::new(self, 0)
    }
}
#[doc = "Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger. 0 -> Secure access only 1 -> Secure + Non-secure access  

You can [`read`](crate::Reg::read) this register and get [`gpio_nsmask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_nsmask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_NSMASK0_SPEC;
impl crate::RegisterSpec for GPIO_NSMASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_nsmask0::R`](R) reader structure"]
impl crate::Readable for GPIO_NSMASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_nsmask0::W`](W) writer structure"]
impl crate::Writable for GPIO_NSMASK0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_NSMASK0 to value 0"]
impl crate::Resettable for GPIO_NSMASK0_SPEC {
    const RESET_VALUE: u32 = 0;
}
