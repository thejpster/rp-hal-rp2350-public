#[doc = "Register `BOOTSEL_LED_CFG` reader"]
pub type R = crate::R<BOOTSEL_LED_CFG_SPEC>;
#[doc = "Register `BOOTSEL_LED_CFG` writer"]
pub type W = crate::W<BOOTSEL_LED_CFG_SPEC>;
#[doc = "Field `PIN` reader - GPIO index to use for bootloader activity LED."]
pub type PIN_R = crate::FieldReader;
#[doc = "Field `ACTIVELOW` reader - LED is active-low. (Default: active-high.)"]
pub type ACTIVELOW_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - GPIO index to use for bootloader activity LED."]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:23 - LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub fn activelow(&self) -> ACTIVELOW_R {
        ACTIVELOW_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_led_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootsel_led_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTSEL_LED_CFG_SPEC;
impl crate::RegisterSpec for BOOTSEL_LED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootsel_led_cfg::R`](R) reader structure"]
impl crate::Readable for BOOTSEL_LED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootsel_led_cfg::W`](W) writer structure"]
impl crate::Writable for BOOTSEL_LED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTSEL_LED_CFG to value 0"]
impl crate::Resettable for BOOTSEL_LED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
