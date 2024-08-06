#[doc = "Register `PROC_IN_SYNC_BYPASS_HI` reader"]
pub type R = crate::R<PROC_IN_SYNC_BYPASS_HI_SPEC>;
#[doc = "Register `PROC_IN_SYNC_BYPASS_HI` writer"]
pub type W = crate::W<PROC_IN_SYNC_BYPASS_HI_SPEC>;
#[doc = "Field `GPIO` reader - "]
pub type GPIO_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO` writer - "]
pub type GPIO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USB_DP` reader - "]
pub type USB_DP_R = crate::BitReader;
#[doc = "Field `USB_DP` writer - "]
pub type USB_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DM` reader - "]
pub type USB_DM_R = crate::BitReader;
#[doc = "Field `USB_DM` writer - "]
pub type USB_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SCK` reader - "]
pub type QSPI_SCK_R = crate::BitReader;
#[doc = "Field `QSPI_SCK` writer - "]
pub type QSPI_SCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_CSN` reader - "]
pub type QSPI_CSN_R = crate::BitReader;
#[doc = "Field `QSPI_CSN` writer - "]
pub type QSPI_CSN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_SD` reader - "]
pub type QSPI_SD_R = crate::FieldReader;
#[doc = "Field `QSPI_SD` writer - "]
pub type QSPI_SD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb_dp(&self) -> USB_DP_R {
        USB_DP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_dm(&self) -> USB_DM_R {
        USB_DM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn qspi_sck(&self) -> QSPI_SCK_R {
        QSPI_SCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn qspi_csn(&self) -> QSPI_CSN_R {
        QSPI_CSN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn qspi_sd(&self) -> QSPI_SD_R {
        QSPI_SD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        GPIO_W::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp(&mut self) -> USB_DP_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        USB_DP_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm(&mut self) -> USB_DM_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        USB_DM_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sck(&mut self) -> QSPI_SCK_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        QSPI_SCK_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_csn(&mut self) -> QSPI_CSN_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        QSPI_CSN_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_sd(&mut self) -> QSPI_SD_W<PROC_IN_SYNC_BYPASS_HI_SPEC> {
        QSPI_SD_W::new(self, 28)
    }
}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63  

You can [`read`](crate::Reg::read) this register and get [`proc_in_sync_bypass_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_in_sync_bypass_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_IN_SYNC_BYPASS_HI_SPEC;
impl crate::RegisterSpec for PROC_IN_SYNC_BYPASS_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_in_sync_bypass_hi::R`](R) reader structure"]
impl crate::Readable for PROC_IN_SYNC_BYPASS_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc_in_sync_bypass_hi::W`](W) writer structure"]
impl crate::Writable for PROC_IN_SYNC_BYPASS_HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROC_IN_SYNC_BYPASS_HI to value 0"]
impl crate::Resettable for PROC_IN_SYNC_BYPASS_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
