#[doc = "Register `NUM_GPIOS` reader"]
pub type R = crate::R<NUM_GPIOS_SPEC>;
#[doc = "Register `NUM_GPIOS` writer"]
pub type W = crate::W<NUM_GPIOS_SPEC>;
#[doc = "Field `NUM_GPIOS` reader - "]
pub type NUM_GPIOS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn num_gpios(&self) -> NUM_GPIOS_R {
        NUM_GPIOS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)  

You can [`read`](crate::Reg::read) this register and get [`num_gpios::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`num_gpios::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NUM_GPIOS_SPEC;
impl crate::RegisterSpec for NUM_GPIOS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`num_gpios::R`](R) reader structure"]
impl crate::Readable for NUM_GPIOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`num_gpios::W`](W) writer structure"]
impl crate::Writable for NUM_GPIOS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NUM_GPIOS to value 0"]
impl crate::Resettable for NUM_GPIOS_SPEC {
    const RESET_VALUE: u16 = 0;
}
