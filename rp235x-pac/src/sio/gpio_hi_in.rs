#[doc = "Register `GPIO_HI_IN` reader"]
pub type R = crate::R<GPIO_HI_IN_SPEC>;
#[doc = "Field `GPIO` reader - Input value on GPIO32...47"]
pub type GPIO_R = crate::FieldReader<u16>;
#[doc = "Field `USB_DP` reader - Input value on USB D+ pin"]
pub type USB_DP_R = crate::BitReader;
#[doc = "Field `USB_DM` reader - Input value on USB D- pin"]
pub type USB_DM_R = crate::BitReader;
#[doc = "Field `QSPI_SCK` reader - Input value on QSPI SCK pin"]
pub type QSPI_SCK_R = crate::BitReader;
#[doc = "Field `QSPI_CSN` reader - Input value on QSPI CSn pin"]
pub type QSPI_CSN_R = crate::BitReader;
#[doc = "Field `QSPI_SD` reader - Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
pub type QSPI_SD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Input value on GPIO32...47"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Input value on USB D+ pin"]
    #[inline(always)]
    pub fn usb_dp(&self) -> USB_DP_R {
        USB_DP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value on USB D- pin"]
    #[inline(always)]
    pub fn usb_dm(&self) -> USB_DM_R {
        USB_DM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value on QSPI SCK pin"]
    #[inline(always)]
    pub fn qspi_sck(&self) -> QSPI_SCK_R {
        QSPI_SCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value on QSPI CSn pin"]
    #[inline(always)]
    pub fn qspi_csn(&self) -> QSPI_CSN_R {
        QSPI_CSN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins"]
    #[inline(always)]
    pub fn qspi_sd(&self) -> QSPI_SD_R {
        QSPI_SD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Input value on GPIO32...47, QSPI IOs and USB pins  

 In the NonSecure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_IN_SPEC;
impl crate::RegisterSpec for GPIO_HI_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hi_in::R`](R) reader structure"]
impl crate::Readable for GPIO_HI_IN_SPEC {}
#[doc = "`reset()` method sets GPIO_HI_IN to value 0"]
impl crate::Resettable for GPIO_HI_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
