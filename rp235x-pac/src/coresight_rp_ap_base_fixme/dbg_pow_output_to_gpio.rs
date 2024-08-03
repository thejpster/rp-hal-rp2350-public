#[doc = "Register `DBG_POW_OUTPUT_TO_GPIO` reader"]
pub type R = crate::R<DBG_POW_OUTPUT_TO_GPIO_SPEC>;
#[doc = "Register `DBG_POW_OUTPUT_TO_GPIO` writer"]
pub type W = crate::W<DBG_POW_OUTPUT_TO_GPIO_SPEC>;
#[doc = "Field `ENABLE` reader - "]
pub type ENABLE_R = crate::FieldReader<u16>;
#[doc = "Field `ENABLE` writer - "]
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DBG_POW_OUTPUT_TO_GPIO_SPEC> {
        ENABLE_W::new(self, 0)
    }
}
#[doc = "Send some, or all, bits of DBG_POW_STATE_SWCORE to gpios.  
 Bit 0 sends bit 0 of DBG_POW_STATE_SWCORE to GPIO 34  
 Bit 1 sends bit 1 of DBG_POW_STATE_SWCORE to GPIO 35  
 Bit 2 sends bit 2 of DBG_POW_STATE_SWCORE to GPIO 36  
 .  
 .  
 Bit 11 sends bit 11 of DBG_POW_STATE_SWCORE to GPIO 45  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_output_to_gpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pow_output_to_gpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_POW_OUTPUT_TO_GPIO_SPEC;
impl crate::RegisterSpec for DBG_POW_OUTPUT_TO_GPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_pow_output_to_gpio::R`](R) reader structure"]
impl crate::Readable for DBG_POW_OUTPUT_TO_GPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_pow_output_to_gpio::W`](W) writer structure"]
impl crate::Writable for DBG_POW_OUTPUT_TO_GPIO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_POW_OUTPUT_TO_GPIO to value 0"]
impl crate::Resettable for DBG_POW_OUTPUT_TO_GPIO_SPEC {
    const RESET_VALUE: u32 = 0;
}
