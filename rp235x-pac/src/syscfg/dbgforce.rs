#[doc = "Register `DBGFORCE` reader"]
pub type R = crate::R<DBGFORCE_SPEC>;
#[doc = "Register `DBGFORCE` writer"]
pub type W = crate::W<DBGFORCE_SPEC>;
#[doc = "Field `SWDO` reader - Observe the value of SWDIO output."]
pub type SWDO_R = crate::BitReader;
#[doc = "Field `SWDI` reader - Directly drive SWDIO input, if ATTACH is set"]
pub type SWDI_R = crate::BitReader;
#[doc = "Field `SWDI` writer - Directly drive SWDIO input, if ATTACH is set"]
pub type SWDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCLK` reader - Directly drive SWCLK, if ATTACH is set"]
pub type SWCLK_R = crate::BitReader;
#[doc = "Field `SWCLK` writer - Directly drive SWCLK, if ATTACH is set"]
pub type SWCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTACH` reader - Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type ATTACH_R = crate::BitReader;
#[doc = "Field `ATTACH` writer - Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
pub type ATTACH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Observe the value of SWDIO output."]
    #[inline(always)]
    pub fn swdo(&self) -> SWDO_R {
        SWDO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Directly drive SWDIO input, if ATTACH is set"]
    #[inline(always)]
    pub fn swdi(&self) -> SWDI_R {
        SWDI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Directly drive SWCLK, if ATTACH is set"]
    #[inline(always)]
    pub fn swclk(&self) -> SWCLK_R {
        SWCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Directly drive SWDIO input, if ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn swdi(&mut self) -> SWDI_W<DBGFORCE_SPEC> {
        SWDI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Directly drive SWCLK, if ATTACH is set"]
    #[inline(always)]
    #[must_use]
    pub fn swclk(&mut self) -> SWCLK_W<DBGFORCE_SPEC> {
        SWCLK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    #[must_use]
    pub fn attach(&mut self) -> ATTACH_W<DBGFORCE_SPEC> {
        ATTACH_W::new(self, 3)
    }
}
#[doc = "Directly control the chip SWD debug port  

You can [`read`](crate::Reg::read) this register and get [`dbgforce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgforce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGFORCE_SPEC;
impl crate::RegisterSpec for DBGFORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgforce::R`](R) reader structure"]
impl crate::Readable for DBGFORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgforce::W`](W) writer structure"]
impl crate::Writable for DBGFORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGFORCE to value 0x06"]
impl crate::Resettable for DBGFORCE_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
