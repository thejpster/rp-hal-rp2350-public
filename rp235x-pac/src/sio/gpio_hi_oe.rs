#[doc = "Register `GPIO_HI_OE` reader"]
pub type R = crate::R<GPIO_HI_OE_SPEC>;
#[doc = "Register `GPIO_HI_OE` writer"]
pub type W = crate::W<GPIO_HI_OE_SPEC>;
#[doc = "Field `GPIO` reader - Output enable value for GPIO32...47"]
pub type GPIO_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO` writer - Output enable value for GPIO32...47"]
pub type GPIO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USB_DP` reader - Output enable value for USB D+ pin"]
pub type USB_DP_R = crate::BitReader;
#[doc = "Field `USB_DP` writer - Output enable value for USB D+ pin"]
pub type USB_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DM` reader - Output enable value for USB D- pin"]
pub type USB_DM_R = crate::BitReader;
#[doc = "Field `USB_DM` writer - Output enable value for USB D- pin"]
pub type USB_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SCK` reader - Output enable value for QSPI SCK pin"]
pub type QSPI_SCK_R = crate::BitReader;
#[doc = "Field `QSPI_SCK` writer - Output enable value for QSPI SCK pin"]
pub type QSPI_SCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_CSN` reader - Output enable value for QSPI CSn pin"]
pub type QSPI_CSN_R = crate::BitReader;
#[doc = "Field `QSPI_CSN` writer - Output enable value for QSPI CSn pin"]
pub type QSPI_CSN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SD` reader - Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
pub type QSPI_SD_R = crate::FieldReader;
#[doc = "Field `QSPI_SD` writer - Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
pub type QSPI_SD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Output enable value for GPIO32...47"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Output enable value for USB D+ pin"]
    #[inline(always)]
    pub fn usb_dp(&self) -> USB_DP_R {
        USB_DP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output enable value for USB D- pin"]
    #[inline(always)]
    pub fn usb_dm(&self) -> USB_DM_R {
        USB_DM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output enable value for QSPI SCK pin"]
    #[inline(always)]
    pub fn qspi_sck(&self) -> QSPI_SCK_R {
        QSPI_SCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output enable value for QSPI CSn pin"]
    #[inline(always)]
    pub fn qspi_csn(&self) -> QSPI_CSN_R {
        QSPI_CSN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
    #[inline(always)]
    pub fn qspi_sd(&self) -> QSPI_SD_R {
        QSPI_SD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output enable value for GPIO32...47"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<GPIO_HI_OE_SPEC> {
        GPIO_W::new(self, 0)
    }
    #[doc = "Bit 24 - Output enable value for USB D+ pin"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp(&mut self) -> USB_DP_W<GPIO_HI_OE_SPEC> {
        USB_DP_W::new(self, 24)
    }
    #[doc = "Bit 25 - Output enable value for USB D- pin"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm(&mut self) -> USB_DM_W<GPIO_HI_OE_SPEC> {
        USB_DM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Output enable value for QSPI SCK pin"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sck(&mut self) -> QSPI_SCK_W<GPIO_HI_OE_SPEC> {
        QSPI_SCK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Output enable value for QSPI CSn pin"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_csn(&mut self) -> QSPI_CSN_W<GPIO_HI_OE_SPEC> {
        QSPI_CSN_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sd(&mut self) -> QSPI_SD_W<GPIO_HI_OE_SPEC> {
        QSPI_SD_W::new(self, 28)
    }
}
#[doc = "Output enable value for GPIO32...47, QSPI IOs and USB pins. Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_oe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_OE_SPEC;
impl crate::RegisterSpec for GPIO_HI_OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hi_oe::R`](R) reader structure"]
impl crate::Readable for GPIO_HI_OE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_hi_oe::W`](W) writer structure"]
impl crate::Writable for GPIO_HI_OE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OE to value 0"]
impl crate::Resettable for GPIO_HI_OE_SPEC {
    const RESET_VALUE: u32 = 0;
}
