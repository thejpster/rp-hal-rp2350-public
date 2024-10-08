#[doc = "Register `ENABLED0` reader"]
pub type R = crate::R<ENABLED0_SPEC>;
#[doc = "Register `ENABLED0` writer"]
pub type W = crate::W<ENABLED0_SPEC>;
#[doc = "Field `CLK_SYS_CLOCKS` reader - "]
pub type CLK_SYS_CLOCKS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ACCESSCTRL` reader - "]
pub type CLK_SYS_ACCESSCTRL_R = crate::BitReader;
#[doc = "Field `CLK_ADC` reader - "]
pub type CLK_ADC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ADC` reader - "]
pub type CLK_SYS_ADC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BOOTRAM` reader - "]
pub type CLK_SYS_BOOTRAM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BUSCTRL` reader - "]
pub type CLK_SYS_BUSCTRL_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BUSFABRIC` reader - "]
pub type CLK_SYS_BUSFABRIC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_DMA` reader - "]
pub type CLK_SYS_DMA_R = crate::BitReader;
#[doc = "Field `CLK_SYS_GLITCH_DETECTOR` reader - "]
pub type CLK_SYS_GLITCH_DETECTOR_R = crate::BitReader;
#[doc = "Field `CLK_HSTX` reader - "]
pub type CLK_HSTX_R = crate::BitReader;
#[doc = "Field `CLK_SYS_HSTX` reader - "]
pub type CLK_SYS_HSTX_R = crate::BitReader;
#[doc = "Field `CLK_SYS_I2C0` reader - "]
pub type CLK_SYS_I2C0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_I2C1` reader - "]
pub type CLK_SYS_I2C1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_IO` reader - "]
pub type CLK_SYS_IO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_JTAG` reader - "]
pub type CLK_SYS_JTAG_R = crate::BitReader;
#[doc = "Field `CLK_REF_OTP` reader - "]
pub type CLK_REF_OTP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_OTP` reader - "]
pub type CLK_SYS_OTP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PADS` reader - "]
pub type CLK_SYS_PADS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO0` reader - "]
pub type CLK_SYS_PIO0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO1` reader - "]
pub type CLK_SYS_PIO1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO2` reader - "]
pub type CLK_SYS_PIO2_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PLL_SYS` reader - "]
pub type CLK_SYS_PLL_SYS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PLL_USB` reader - "]
pub type CLK_SYS_PLL_USB_R = crate::BitReader;
#[doc = "Field `CLK_REF_POWMAN` reader - "]
pub type CLK_REF_POWMAN_R = crate::BitReader;
#[doc = "Field `CLK_SYS_POWMAN` reader - "]
pub type CLK_SYS_POWMAN_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PWM` reader - "]
pub type CLK_SYS_PWM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_RESETS` reader - "]
pub type CLK_SYS_RESETS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ROM` reader - "]
pub type CLK_SYS_ROM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ROSC` reader - "]
pub type CLK_SYS_ROSC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PSM` reader - "]
pub type CLK_SYS_PSM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SHA256` reader - "]
pub type CLK_SYS_SHA256_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SIO` reader - "]
pub type CLK_SYS_SIO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> CLK_SYS_CLOCKS_R {
        CLK_SYS_CLOCKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_sys_accessctrl(&self) -> CLK_SYS_ACCESSCTRL_R {
        CLK_SYS_ACCESSCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_adc(&self) -> CLK_ADC_R {
        CLK_ADC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> CLK_SYS_ADC_R {
        CLK_SYS_ADC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_bootram(&self) -> CLK_SYS_BOOTRAM_R {
        CLK_SYS_BOOTRAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> CLK_SYS_BUSCTRL_R {
        CLK_SYS_BUSCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> CLK_SYS_BUSFABRIC_R {
        CLK_SYS_BUSFABRIC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> CLK_SYS_DMA_R {
        CLK_SYS_DMA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_sys_glitch_detector(&self) -> CLK_SYS_GLITCH_DETECTOR_R {
        CLK_SYS_GLITCH_DETECTOR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_hstx(&self) -> CLK_HSTX_R {
        CLK_HSTX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_hstx(&self) -> CLK_SYS_HSTX_R {
        CLK_SYS_HSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> CLK_SYS_I2C0_R {
        CLK_SYS_I2C0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> CLK_SYS_I2C1_R {
        CLK_SYS_I2C1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_io(&self) -> CLK_SYS_IO_R {
        CLK_SYS_IO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> CLK_SYS_JTAG_R {
        CLK_SYS_JTAG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_ref_otp(&self) -> CLK_REF_OTP_R {
        CLK_REF_OTP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_sys_otp(&self) -> CLK_SYS_OTP_R {
        CLK_SYS_OTP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> CLK_SYS_PADS_R {
        CLK_SYS_PADS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> CLK_SYS_PIO0_R {
        CLK_SYS_PIO0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> CLK_SYS_PIO1_R {
        CLK_SYS_PIO1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_sys_pio2(&self) -> CLK_SYS_PIO2_R {
        CLK_SYS_PIO2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> CLK_SYS_PLL_SYS_R {
        CLK_SYS_PLL_SYS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> CLK_SYS_PLL_USB_R {
        CLK_SYS_PLL_USB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_ref_powman(&self) -> CLK_REF_POWMAN_R {
        CLK_REF_POWMAN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_sys_powman(&self) -> CLK_SYS_POWMAN_R {
        CLK_SYS_POWMAN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> CLK_SYS_PWM_R {
        CLK_SYS_PWM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> CLK_SYS_RESETS_R {
        CLK_SYS_RESETS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> CLK_SYS_ROM_R {
        CLK_SYS_ROM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> CLK_SYS_ROSC_R {
        CLK_SYS_ROSC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> CLK_SYS_PSM_R {
        CLK_SYS_PSM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_sys_sha256(&self) -> CLK_SYS_SHA256_R {
        CLK_SYS_SHA256_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> CLK_SYS_SIO_R {
        CLK_SYS_SIO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "indicates the state of the clock enable  

You can [`read`](crate::Reg::read) this register and get [`enabled0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enabled0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLED0_SPEC;
impl crate::RegisterSpec for ENABLED0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enabled0::R`](R) reader structure"]
impl crate::Readable for ENABLED0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enabled0::W`](W) writer structure"]
impl crate::Writable for ENABLED0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLED0 to value 0"]
impl crate::Resettable for ENABLED0_SPEC {
    const RESET_VALUE: u32 = 0;
}
