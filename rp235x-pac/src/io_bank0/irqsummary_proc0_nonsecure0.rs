#[doc = "Register `IRQSUMMARY_PROC0_NONSECURE0` reader"]
pub type R = crate::R<IRQSUMMARY_PROC0_NONSECURE0_SPEC>;
#[doc = "Register `IRQSUMMARY_PROC0_NONSECURE0` writer"]
pub type W = crate::W<IRQSUMMARY_PROC0_NONSECURE0_SPEC>;
#[doc = "Field `GPIO0` reader - "]
pub type GPIO0_R = crate::BitReader;
#[doc = "Field `GPIO1` reader - "]
pub type GPIO1_R = crate::BitReader;
#[doc = "Field `GPIO2` reader - "]
pub type GPIO2_R = crate::BitReader;
#[doc = "Field `GPIO3` reader - "]
pub type GPIO3_R = crate::BitReader;
#[doc = "Field `GPIO4` reader - "]
pub type GPIO4_R = crate::BitReader;
#[doc = "Field `GPIO5` reader - "]
pub type GPIO5_R = crate::BitReader;
#[doc = "Field `GPIO6` reader - "]
pub type GPIO6_R = crate::BitReader;
#[doc = "Field `GPIO7` reader - "]
pub type GPIO7_R = crate::BitReader;
#[doc = "Field `GPIO8` reader - "]
pub type GPIO8_R = crate::BitReader;
#[doc = "Field `GPIO9` reader - "]
pub type GPIO9_R = crate::BitReader;
#[doc = "Field `GPIO10` reader - "]
pub type GPIO10_R = crate::BitReader;
#[doc = "Field `GPIO11` reader - "]
pub type GPIO11_R = crate::BitReader;
#[doc = "Field `GPIO12` reader - "]
pub type GPIO12_R = crate::BitReader;
#[doc = "Field `GPIO13` reader - "]
pub type GPIO13_R = crate::BitReader;
#[doc = "Field `GPIO14` reader - "]
pub type GPIO14_R = crate::BitReader;
#[doc = "Field `GPIO15` reader - "]
pub type GPIO15_R = crate::BitReader;
#[doc = "Field `GPIO16` reader - "]
pub type GPIO16_R = crate::BitReader;
#[doc = "Field `GPIO17` reader - "]
pub type GPIO17_R = crate::BitReader;
#[doc = "Field `GPIO18` reader - "]
pub type GPIO18_R = crate::BitReader;
#[doc = "Field `GPIO19` reader - "]
pub type GPIO19_R = crate::BitReader;
#[doc = "Field `GPIO20` reader - "]
pub type GPIO20_R = crate::BitReader;
#[doc = "Field `GPIO21` reader - "]
pub type GPIO21_R = crate::BitReader;
#[doc = "Field `GPIO22` reader - "]
pub type GPIO22_R = crate::BitReader;
#[doc = "Field `GPIO23` reader - "]
pub type GPIO23_R = crate::BitReader;
#[doc = "Field `GPIO24` reader - "]
pub type GPIO24_R = crate::BitReader;
#[doc = "Field `GPIO25` reader - "]
pub type GPIO25_R = crate::BitReader;
#[doc = "Field `GPIO26` reader - "]
pub type GPIO26_R = crate::BitReader;
#[doc = "Field `GPIO27` reader - "]
pub type GPIO27_R = crate::BitReader;
#[doc = "Field `GPIO28` reader - "]
pub type GPIO28_R = crate::BitReader;
#[doc = "Field `GPIO29` reader - "]
pub type GPIO29_R = crate::BitReader;
#[doc = "Field `GPIO30` reader - "]
pub type GPIO30_R = crate::BitReader;
#[doc = "Field `GPIO31` reader - "]
pub type GPIO31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpio10(&self) -> GPIO10_R {
        GPIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpio11(&self) -> GPIO11_R {
        GPIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpio12(&self) -> GPIO12_R {
        GPIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpio13(&self) -> GPIO13_R {
        GPIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpio14(&self) -> GPIO14_R {
        GPIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpio15(&self) -> GPIO15_R {
        GPIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpio16(&self) -> GPIO16_R {
        GPIO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpio17(&self) -> GPIO17_R {
        GPIO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpio18(&self) -> GPIO18_R {
        GPIO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio19(&self) -> GPIO19_R {
        GPIO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpio20(&self) -> GPIO20_R {
        GPIO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn gpio21(&self) -> GPIO21_R {
        GPIO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn gpio22(&self) -> GPIO22_R {
        GPIO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpio23(&self) -> GPIO23_R {
        GPIO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpio24(&self) -> GPIO24_R {
        GPIO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpio25(&self) -> GPIO25_R {
        GPIO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio26(&self) -> GPIO26_R {
        GPIO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio27(&self) -> GPIO27_R {
        GPIO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio28(&self) -> GPIO28_R {
        GPIO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio29(&self) -> GPIO29_R {
        GPIO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio30(&self) -> GPIO30_R {
        GPIO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio31(&self) -> GPIO31_R {
        GPIO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`irqsummary_proc0_nonsecure0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsummary_proc0_nonsecure0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQSUMMARY_PROC0_NONSECURE0_SPEC;
impl crate::RegisterSpec for IRQSUMMARY_PROC0_NONSECURE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqsummary_proc0_nonsecure0::R`](R) reader structure"]
impl crate::Readable for IRQSUMMARY_PROC0_NONSECURE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqsummary_proc0_nonsecure0::W`](W) writer structure"]
impl crate::Writable for IRQSUMMARY_PROC0_NONSECURE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSUMMARY_PROC0_NONSECURE0 to value 0"]
impl crate::Resettable for IRQSUMMARY_PROC0_NONSECURE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
