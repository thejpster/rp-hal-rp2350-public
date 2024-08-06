#[doc = "Register `GPIOBASE` reader"]
pub type R = crate::R<GPIOBASE_SPEC>;
#[doc = "Register `GPIOBASE` writer"]
pub type W = crate::W<GPIOBASE_SPEC>;
#[doc = "Field `GPIOBASE` reader - "]
pub type GPIOBASE_R = crate::BitReader;
#[doc = "Field `GPIOBASE` writer - "]
pub type GPIOBASE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpiobase(&self) -> GPIOBASE_R {
        GPIOBASE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobase(&mut self) -> GPIOBASE_W<GPIOBASE_SPEC> {
        GPIOBASE_W::new(self, 4)
    }
}
#[doc = "Relocate GPIO 0 (from PIO's point of view) in the system GPIO numbering, to access more than 32 GPIOs from PIO. Only the values 0 and 16 are supported (only bit 4 is writable).  

You can [`read`](crate::Reg::read) this register and get [`gpiobase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiobase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOBASE_SPEC;
impl crate::RegisterSpec for GPIOBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiobase::R`](R) reader structure"]
impl crate::Readable for GPIOBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpiobase::W`](W) writer structure"]
impl crate::Writable for GPIOBASE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOBASE to value 0"]
impl crate::Resettable for GPIOBASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
