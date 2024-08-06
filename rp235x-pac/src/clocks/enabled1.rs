#[doc = "Register `ENABLED1` reader"]
pub type R = crate::R<ENABLED1_SPEC>;
#[doc = "Register `ENABLED1` writer"]
pub type W = crate::W<ENABLED1_SPEC>;
#[doc = "Field `CLK_PERI_SPI0` reader - "]
pub type CLK_PERI_SPI0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SPI0` reader - "]
pub type CLK_SYS_SPI0_R = crate::BitReader;
#[doc = "Field `CLK_PERI_SPI1` reader - "]
pub type CLK_PERI_SPI1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SPI1` reader - "]
pub type CLK_SYS_SPI1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM0` reader - "]
pub type CLK_SYS_SRAM0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM1` reader - "]
pub type CLK_SYS_SRAM1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM2` reader - "]
pub type CLK_SYS_SRAM2_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM3` reader - "]
pub type CLK_SYS_SRAM3_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM4` reader - "]
pub type CLK_SYS_SRAM4_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM5` reader - "]
pub type CLK_SYS_SRAM5_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM6` reader - "]
pub type CLK_SYS_SRAM6_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM7` reader - "]
pub type CLK_SYS_SRAM7_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM8` reader - "]
pub type CLK_SYS_SRAM8_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SRAM9` reader - "]
pub type CLK_SYS_SRAM9_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSCFG` reader - "]
pub type CLK_SYS_SYSCFG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_SYSINFO` reader - "]
pub type CLK_SYS_SYSINFO_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TBMAN` reader - "]
pub type CLK_SYS_TBMAN_R = crate::BitReader;
#[doc = "Field `CLK_REF_TICKS` reader - "]
pub type CLK_REF_TICKS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TICKS` reader - "]
pub type CLK_SYS_TICKS_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TIMER0` reader - "]
pub type CLK_SYS_TIMER0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TIMER1` reader - "]
pub type CLK_SYS_TIMER1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_TRNG` reader - "]
pub type CLK_SYS_TRNG_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART0` reader - "]
pub type CLK_PERI_UART0_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART0` reader - "]
pub type CLK_SYS_UART0_R = crate::BitReader;
#[doc = "Field `CLK_PERI_UART1` reader - "]
pub type CLK_PERI_UART1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_UART1` reader - "]
pub type CLK_SYS_UART1_R = crate::BitReader;
#[doc = "Field `CLK_SYS_USBCTRL` reader - "]
pub type CLK_SYS_USBCTRL_R = crate::BitReader;
#[doc = "Field `CLK_USB` reader - "]
pub type CLK_USB_R = crate::BitReader;
#[doc = "Field `CLK_SYS_WATCHDOG` reader - "]
pub type CLK_SYS_WATCHDOG_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XIP` reader - "]
pub type CLK_SYS_XIP_R = crate::BitReader;
#[doc = "Field `CLK_SYS_XOSC` reader - "]
pub type CLK_SYS_XOSC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> CLK_PERI_SPI0_R {
        CLK_PERI_SPI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> CLK_SYS_SPI0_R {
        CLK_SYS_SPI0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> CLK_PERI_SPI1_R {
        CLK_PERI_SPI1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> CLK_SYS_SPI1_R {
        CLK_SYS_SPI1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> CLK_SYS_SRAM0_R {
        CLK_SYS_SRAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> CLK_SYS_SRAM1_R {
        CLK_SYS_SRAM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> CLK_SYS_SRAM2_R {
        CLK_SYS_SRAM2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> CLK_SYS_SRAM3_R {
        CLK_SYS_SRAM3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> CLK_SYS_SRAM4_R {
        CLK_SYS_SRAM4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> CLK_SYS_SRAM5_R {
        CLK_SYS_SRAM5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_sys_sram6(&self) -> CLK_SYS_SRAM6_R {
        CLK_SYS_SRAM6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_sys_sram7(&self) -> CLK_SYS_SRAM7_R {
        CLK_SYS_SRAM7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_sys_sram8(&self) -> CLK_SYS_SRAM8_R {
        CLK_SYS_SRAM8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_sys_sram9(&self) -> CLK_SYS_SRAM9_R {
        CLK_SYS_SRAM9_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> CLK_SYS_SYSCFG_R {
        CLK_SYS_SYSCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> CLK_SYS_SYSINFO_R {
        CLK_SYS_SYSINFO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> CLK_SYS_TBMAN_R {
        CLK_SYS_TBMAN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_ref_ticks(&self) -> CLK_REF_TICKS_R {
        CLK_REF_TICKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sys_ticks(&self) -> CLK_SYS_TICKS_R {
        CLK_SYS_TICKS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_sys_timer0(&self) -> CLK_SYS_TIMER0_R {
        CLK_SYS_TIMER0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_sys_timer1(&self) -> CLK_SYS_TIMER1_R {
        CLK_SYS_TIMER1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_sys_trng(&self) -> CLK_SYS_TRNG_R {
        CLK_SYS_TRNG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> CLK_PERI_UART0_R {
        CLK_PERI_UART0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> CLK_SYS_UART0_R {
        CLK_SYS_UART0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> CLK_PERI_UART1_R {
        CLK_PERI_UART1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> CLK_SYS_UART1_R {
        CLK_SYS_UART1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> CLK_SYS_USBCTRL_R {
        CLK_SYS_USBCTRL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_usb(&self) -> CLK_USB_R {
        CLK_USB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> CLK_SYS_WATCHDOG_R {
        CLK_SYS_WATCHDOG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> CLK_SYS_XIP_R {
        CLK_SYS_XIP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> CLK_SYS_XOSC_R {
        CLK_SYS_XOSC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {}
#[doc = "indicates the state of the clock enable  

You can [`read`](crate::Reg::read) this register and get [`enabled1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enabled1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLED1_SPEC;
impl crate::RegisterSpec for ENABLED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enabled1::R`](R) reader structure"]
impl crate::Readable for ENABLED1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enabled1::W`](W) writer structure"]
impl crate::Writable for ENABLED1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLED1 to value 0"]
impl crate::Resettable for ENABLED1_SPEC {
    const RESET_VALUE: u32 = 0;
}
