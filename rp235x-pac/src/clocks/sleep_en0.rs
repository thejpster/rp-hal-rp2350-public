#[doc = "Register `SLEEP_EN0` reader"]
pub type R = crate::R<SLEEP_EN0_SPEC>;
#[doc = "Register `SLEEP_EN0` writer"]
pub type W = crate::W<SLEEP_EN0_SPEC>;
#[doc = "Field `CLK_SYS_CLOCKS` reader - "]
pub type CLK_SYS_CLOCKS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_CLOCKS` writer - "]
pub type CLK_SYS_CLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_ACCESSCTRL` reader - "]
pub type CLK_SYS_ACCESSCTRL_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ACCESSCTRL` writer - "]
pub type CLK_SYS_ACCESSCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ADC` reader - "]
pub type CLK_ADC_R = crate::BitReader;
#[doc = "Field `CLK_ADC` writer - "]
pub type CLK_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_ADC` reader - "]
pub type CLK_SYS_ADC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ADC` writer - "]
pub type CLK_SYS_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_BOOTRAM` reader - "]
pub type CLK_SYS_BOOTRAM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BOOTRAM` writer - "]
pub type CLK_SYS_BOOTRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_BUSCTRL` reader - "]
pub type CLK_SYS_BUSCTRL_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BUSCTRL` writer - "]
pub type CLK_SYS_BUSCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_BUSFABRIC` reader - "]
pub type CLK_SYS_BUSFABRIC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_BUSFABRIC` writer - "]
pub type CLK_SYS_BUSFABRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_DMA` reader - "]
pub type CLK_SYS_DMA_R = crate::BitReader;
#[doc = "Field `CLK_SYS_DMA` writer - "]
pub type CLK_SYS_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_GLITCH_DETECTOR` reader - "]
pub type CLK_SYS_GLITCH_DETECTOR_R = crate::BitReader;
#[doc = "Field `CLK_SYS_GLITCH_DETECTOR` writer - "]
pub type CLK_SYS_GLITCH_DETECTOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HSTX` reader - "]
pub type CLK_HSTX_R = crate::BitReader;
#[doc = "Field `CLK_HSTX` writer - "]
pub type CLK_HSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_HSTX` reader - "]
pub type CLK_SYS_HSTX_R = crate::BitReader;
#[doc = "Field `CLK_SYS_HSTX` writer - "]
pub type CLK_SYS_HSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_I2C0` reader - "]
pub type CLK_SYS_I2C0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_I2C0` writer - "]
pub type CLK_SYS_I2C0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_I2C1` reader - "]
pub type CLK_SYS_I2C1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_I2C1` writer - "]
pub type CLK_SYS_I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_IO` reader - "]
pub type CLK_SYS_IO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_IO` writer - "]
pub type CLK_SYS_IO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_JTAG` reader - "]
pub type CLK_SYS_JTAG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_JTAG` writer - "]
pub type CLK_SYS_JTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_REF_OTP` reader - "]
pub type CLK_REF_OTP_R = crate::BitReader;
#[doc = "Field `CLK_REF_OTP` writer - "]
pub type CLK_REF_OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_OTP` reader - "]
pub type CLK_SYS_OTP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_OTP` writer - "]
pub type CLK_SYS_OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PADS` reader - "]
pub type CLK_SYS_PADS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PADS` writer - "]
pub type CLK_SYS_PADS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PIO0` reader - "]
pub type CLK_SYS_PIO0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO0` writer - "]
pub type CLK_SYS_PIO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PIO1` reader - "]
pub type CLK_SYS_PIO1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO1` writer - "]
pub type CLK_SYS_PIO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PIO2` reader - "]
pub type CLK_SYS_PIO2_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PIO2` writer - "]
pub type CLK_SYS_PIO2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PLL_SYS` reader - "]
pub type CLK_SYS_PLL_SYS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PLL_SYS` writer - "]
pub type CLK_SYS_PLL_SYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PLL_USB` reader - "]
pub type CLK_SYS_PLL_USB_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PLL_USB` writer - "]
pub type CLK_SYS_PLL_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_REF_POWMAN` reader - "]
pub type CLK_REF_POWMAN_R = crate::BitReader;
#[doc = "Field `CLK_REF_POWMAN` writer - "]
pub type CLK_REF_POWMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_POWMAN` reader - "]
pub type CLK_SYS_POWMAN_R = crate::BitReader;
#[doc = "Field `CLK_SYS_POWMAN` writer - "]
pub type CLK_SYS_POWMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PWM` reader - "]
pub type CLK_SYS_PWM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PWM` writer - "]
pub type CLK_SYS_PWM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_RESETS` reader - "]
pub type CLK_SYS_RESETS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_RESETS` writer - "]
pub type CLK_SYS_RESETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_ROM` reader - "]
pub type CLK_SYS_ROM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ROM` writer - "]
pub type CLK_SYS_ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_ROSC` reader - "]
pub type CLK_SYS_ROSC_R = crate::BitReader;
#[doc = "Field `CLK_SYS_ROSC` writer - "]
pub type CLK_SYS_ROSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_PSM` reader - "]
pub type CLK_SYS_PSM_R = crate::BitReader;
#[doc = "Field `CLK_SYS_PSM` writer - "]
pub type CLK_SYS_PSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_SHA256` reader - "]
pub type CLK_SYS_SHA256_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SHA256` writer - "]
pub type CLK_SYS_SHA256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SYS_SIO` reader - "]
pub type CLK_SYS_SIO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SIO` writer - "]
pub type CLK_SYS_SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_clocks(&mut self) -> CLK_SYS_CLOCKS_W<SLEEP_EN0_SPEC> {
        CLK_SYS_CLOCKS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_accessctrl(&mut self) -> CLK_SYS_ACCESSCTRL_W<SLEEP_EN0_SPEC> {
        CLK_SYS_ACCESSCTRL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_adc(&mut self) -> CLK_ADC_W<SLEEP_EN0_SPEC> {
        CLK_ADC_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_adc(&mut self) -> CLK_SYS_ADC_W<SLEEP_EN0_SPEC> {
        CLK_SYS_ADC_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_bootram(&mut self) -> CLK_SYS_BOOTRAM_W<SLEEP_EN0_SPEC> {
        CLK_SYS_BOOTRAM_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_busctrl(&mut self) -> CLK_SYS_BUSCTRL_W<SLEEP_EN0_SPEC> {
        CLK_SYS_BUSCTRL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_busfabric(&mut self) -> CLK_SYS_BUSFABRIC_W<SLEEP_EN0_SPEC> {
        CLK_SYS_BUSFABRIC_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_dma(&mut self) -> CLK_SYS_DMA_W<SLEEP_EN0_SPEC> {
        CLK_SYS_DMA_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_glitch_detector(&mut self) -> CLK_SYS_GLITCH_DETECTOR_W<SLEEP_EN0_SPEC> {
        CLK_SYS_GLITCH_DETECTOR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hstx(&mut self) -> CLK_HSTX_W<SLEEP_EN0_SPEC> {
        CLK_HSTX_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_hstx(&mut self) -> CLK_SYS_HSTX_W<SLEEP_EN0_SPEC> {
        CLK_SYS_HSTX_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_i2c0(&mut self) -> CLK_SYS_I2C0_W<SLEEP_EN0_SPEC> {
        CLK_SYS_I2C0_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_i2c1(&mut self) -> CLK_SYS_I2C1_W<SLEEP_EN0_SPEC> {
        CLK_SYS_I2C1_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_io(&mut self) -> CLK_SYS_IO_W<SLEEP_EN0_SPEC> {
        CLK_SYS_IO_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_jtag(&mut self) -> CLK_SYS_JTAG_W<SLEEP_EN0_SPEC> {
        CLK_SYS_JTAG_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ref_otp(&mut self) -> CLK_REF_OTP_W<SLEEP_EN0_SPEC> {
        CLK_REF_OTP_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_otp(&mut self) -> CLK_SYS_OTP_W<SLEEP_EN0_SPEC> {
        CLK_SYS_OTP_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pads(&mut self) -> CLK_SYS_PADS_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PADS_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pio0(&mut self) -> CLK_SYS_PIO0_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PIO0_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pio1(&mut self) -> CLK_SYS_PIO1_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PIO1_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pio2(&mut self) -> CLK_SYS_PIO2_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PIO2_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pll_sys(&mut self) -> CLK_SYS_PLL_SYS_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PLL_SYS_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pll_usb(&mut self) -> CLK_SYS_PLL_USB_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PLL_USB_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ref_powman(&mut self) -> CLK_REF_POWMAN_W<SLEEP_EN0_SPEC> {
        CLK_REF_POWMAN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_powman(&mut self) -> CLK_SYS_POWMAN_W<SLEEP_EN0_SPEC> {
        CLK_SYS_POWMAN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_pwm(&mut self) -> CLK_SYS_PWM_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PWM_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_resets(&mut self) -> CLK_SYS_RESETS_W<SLEEP_EN0_SPEC> {
        CLK_SYS_RESETS_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_rom(&mut self) -> CLK_SYS_ROM_W<SLEEP_EN0_SPEC> {
        CLK_SYS_ROM_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_rosc(&mut self) -> CLK_SYS_ROSC_W<SLEEP_EN0_SPEC> {
        CLK_SYS_ROSC_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_psm(&mut self) -> CLK_SYS_PSM_W<SLEEP_EN0_SPEC> {
        CLK_SYS_PSM_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sha256(&mut self) -> CLK_SYS_SHA256_W<SLEEP_EN0_SPEC> {
        CLK_SYS_SHA256_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys_sio(&mut self) -> CLK_SYS_SIO_W<SLEEP_EN0_SPEC> {
        CLK_SYS_SIO_W::new(self, 31)
    }
}
#[doc = "enable clock in sleep mode  

You can [`read`](crate::Reg::read) this register and get [`sleep_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_EN0_SPEC;
impl crate::RegisterSpec for SLEEP_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_en0::R`](R) reader structure"]
impl crate::Readable for SLEEP_EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_en0::W`](W) writer structure"]
impl crate::Writable for SLEEP_EN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEP_EN0 to value 0xffff_ffff"]
impl crate::Resettable for SLEEP_EN0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
