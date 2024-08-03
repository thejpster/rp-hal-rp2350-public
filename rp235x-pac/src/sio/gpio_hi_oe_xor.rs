#[doc = "Register `GPIO_HI_OE_XOR` writer"]
pub type W = crate::W<GPIO_HI_OE_XOR_SPEC>;
#[doc = "Field `GPIO` writer - "]
pub type GPIO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USB_DP` writer - "]
pub type USB_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DM` writer - "]
pub type USB_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SCK` writer - "]
pub type QSPI_SCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_CSN` writer - "]
pub type QSPI_CSN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SD` writer - "]
pub type QSPI_SD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<GPIO_HI_OE_XOR_SPEC> {
        GPIO_W::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp(&mut self) -> USB_DP_W<GPIO_HI_OE_XOR_SPEC> {
        USB_DP_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm(&mut self) -> USB_DM_W<GPIO_HI_OE_XOR_SPEC> {
        USB_DM_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sck(&mut self) -> QSPI_SCK_W<GPIO_HI_OE_XOR_SPEC> {
        QSPI_SCK_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_csn(&mut self) -> QSPI_CSN_W<GPIO_HI_OE_XOR_SPEC> {
        QSPI_CSN_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sd(&mut self) -> QSPI_SD_W<GPIO_HI_OE_XOR_SPEC> {
        QSPI_SD_W::new(self, 28)
    }
}
#[doc = "Output enable XOR for GPIO32...47, QSPI IOs and USB pins.  
 Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe_xor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_OE_XOR_SPEC;
impl crate::RegisterSpec for GPIO_HI_OE_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_hi_oe_xor::W`](W) writer structure"]
impl crate::Writable for GPIO_HI_OE_XOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OE_XOR to value 0"]
impl crate::Resettable for GPIO_HI_OE_XOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
