#[doc = "Register `IRQSUMMARY_PROC1_SECURE` reader"]
pub type R = crate::R<IRQSUMMARY_PROC1_SECURE_SPEC>;
#[doc = "Field `USBPHY_DP` reader - "]
pub type USBPHY_DP_R = crate::BitReader;
#[doc = "Field `USBPHY_DM` reader - "]
pub type USBPHY_DM_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SCLK` reader - "]
pub type GPIO_QSPI_SCLK_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SS` reader - "]
pub type GPIO_QSPI_SS_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SD0` reader - "]
pub type GPIO_QSPI_SD0_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SD1` reader - "]
pub type GPIO_QSPI_SD1_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SD2` reader - "]
pub type GPIO_QSPI_SD2_R = crate::BitReader;
#[doc = "Field `GPIO_QSPI_SD3` reader - "]
pub type GPIO_QSPI_SD3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> USBPHY_DP_R {
        USBPHY_DP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> USBPHY_DM_R {
        USBPHY_DM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> GPIO_QSPI_SCLK_R {
        GPIO_QSPI_SCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> GPIO_QSPI_SS_R {
        GPIO_QSPI_SS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> GPIO_QSPI_SD0_R {
        GPIO_QSPI_SD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> GPIO_QSPI_SD1_R {
        GPIO_QSPI_SD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> GPIO_QSPI_SD2_R {
        GPIO_QSPI_SD2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> GPIO_QSPI_SD3_R {
        GPIO_QSPI_SD3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc1_secure::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQSUMMARY_PROC1_SECURE_SPEC;
impl crate::RegisterSpec for IRQSUMMARY_PROC1_SECURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqsummary_proc1_secure::R`](R) reader structure"]
impl crate::Readable for IRQSUMMARY_PROC1_SECURE_SPEC {}
#[doc = "`reset()` method sets IRQSUMMARY_PROC1_SECURE to value 0"]
impl crate::Resettable for IRQSUMMARY_PROC1_SECURE_SPEC {
    const RESET_VALUE: u32 = 0;
}
