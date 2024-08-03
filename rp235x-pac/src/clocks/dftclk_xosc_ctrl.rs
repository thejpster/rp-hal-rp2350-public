#[doc = "Register `DFTCLK_XOSC_CTRL` reader"]
pub type R = crate::R<DFTCLK_XOSC_CTRL_SPEC>;
#[doc = "Register `DFTCLK_XOSC_CTRL` writer"]
pub type W = crate::W<DFTCLK_XOSC_CTRL_SPEC>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: `0`"]
    NULL = 0,
    #[doc = "1: `1`"]
    CLKSRC_PLL_USB_PRIMARY = 1,
    #[doc = "2: `10`"]
    CLKSRC_GPIN0 = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRC_A {
    type Ux = u8;
}
impl crate::IsEnum for SRC_A {}
#[doc = "Field `SRC` reader - "]
pub type SRC_R = crate::FieldReader<SRC_A>;
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::NULL),
            1 => Some(SRC_A::CLKSRC_PLL_USB_PRIMARY),
            2 => Some(SRC_A::CLKSRC_GPIN0),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == SRC_A::NULL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clksrc_pll_usb_primary(&self) -> bool {
        *self == SRC_A::CLKSRC_PLL_USB_PRIMARY
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_clksrc_gpin0(&self) -> bool {
        *self == SRC_A::CLKSRC_GPIN0
    }
}
#[doc = "Field `SRC` writer - "]
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRC_A>;
impl<'a, REG> SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::NULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clksrc_pll_usb_primary(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::CLKSRC_PLL_USB_PRIMARY)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clksrc_gpin0(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::CLKSRC_GPIN0)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<DFTCLK_XOSC_CTRL_SPEC> {
        SRC_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`dftclk_xosc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dftclk_xosc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFTCLK_XOSC_CTRL_SPEC;
impl crate::RegisterSpec for DFTCLK_XOSC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftclk_xosc_ctrl::R`](R) reader structure"]
impl crate::Readable for DFTCLK_XOSC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dftclk_xosc_ctrl::W`](W) writer structure"]
impl crate::Writable for DFTCLK_XOSC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTCLK_XOSC_CTRL to value 0"]
impl crate::Resettable for DFTCLK_XOSC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
