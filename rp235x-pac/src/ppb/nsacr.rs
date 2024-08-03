#[doc = "Register `NSACR` reader"]
pub type R = crate::R<NSACR_SPEC>;
#[doc = "Register `NSACR` writer"]
pub type W = crate::W<NSACR_SPEC>;
#[doc = "Field `CP0` reader - Enables Non-secure access to coprocessor CP0"]
pub type CP0_R = crate::BitReader;
#[doc = "Field `CP0` writer - Enables Non-secure access to coprocessor CP0"]
pub type CP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1` reader - Enables Non-secure access to coprocessor CP1"]
pub type CP1_R = crate::BitReader;
#[doc = "Field `CP1` writer - Enables Non-secure access to coprocessor CP1"]
pub type CP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP2` reader - Enables Non-secure access to coprocessor CP2"]
pub type CP2_R = crate::BitReader;
#[doc = "Field `CP2` writer - Enables Non-secure access to coprocessor CP2"]
pub type CP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP3` reader - Enables Non-secure access to coprocessor CP3"]
pub type CP3_R = crate::BitReader;
#[doc = "Field `CP3` writer - Enables Non-secure access to coprocessor CP3"]
pub type CP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP4` reader - Enables Non-secure access to coprocessor CP4"]
pub type CP4_R = crate::BitReader;
#[doc = "Field `CP4` writer - Enables Non-secure access to coprocessor CP4"]
pub type CP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP5` reader - Enables Non-secure access to coprocessor CP5"]
pub type CP5_R = crate::BitReader;
#[doc = "Field `CP5` writer - Enables Non-secure access to coprocessor CP5"]
pub type CP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP6` reader - Enables Non-secure access to coprocessor CP6"]
pub type CP6_R = crate::BitReader;
#[doc = "Field `CP6` writer - Enables Non-secure access to coprocessor CP6"]
pub type CP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP7` reader - Enables Non-secure access to coprocessor CP7"]
pub type CP7_R = crate::BitReader;
#[doc = "Field `CP7` writer - Enables Non-secure access to coprocessor CP7"]
pub type CP7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP10` reader - Enables Non-secure access to the Floating-point Extension"]
pub type CP10_R = crate::BitReader;
#[doc = "Field `CP10` writer - Enables Non-secure access to the Floating-point Extension"]
pub type CP10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP11` reader - Enables Non-secure access to the Floating-point Extension"]
pub type CP11_R = crate::BitReader;
#[doc = "Field `CP11` writer - Enables Non-secure access to the Floating-point Extension"]
pub type CP11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables Non-secure access to coprocessor CP0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables Non-secure access to coprocessor CP1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables Non-secure access to coprocessor CP2"]
    #[inline(always)]
    pub fn cp2(&self) -> CP2_R {
        CP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables Non-secure access to coprocessor CP3"]
    #[inline(always)]
    pub fn cp3(&self) -> CP3_R {
        CP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables Non-secure access to coprocessor CP4"]
    #[inline(always)]
    pub fn cp4(&self) -> CP4_R {
        CP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables Non-secure access to coprocessor CP5"]
    #[inline(always)]
    pub fn cp5(&self) -> CP5_R {
        CP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables Non-secure access to coprocessor CP6"]
    #[inline(always)]
    pub fn cp6(&self) -> CP6_R {
        CP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables Non-secure access to coprocessor CP7"]
    #[inline(always)]
    pub fn cp7(&self) -> CP7_R {
        CP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables Non-secure access to the Floating-point Extension"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables Non-secure access to the Floating-point Extension"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables Non-secure access to coprocessor CP0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<NSACR_SPEC> {
        CP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables Non-secure access to coprocessor CP1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<NSACR_SPEC> {
        CP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables Non-secure access to coprocessor CP2"]
    #[inline(always)]
    #[must_use]
    pub fn cp2(&mut self) -> CP2_W<NSACR_SPEC> {
        CP2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables Non-secure access to coprocessor CP3"]
    #[inline(always)]
    #[must_use]
    pub fn cp3(&mut self) -> CP3_W<NSACR_SPEC> {
        CP3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables Non-secure access to coprocessor CP4"]
    #[inline(always)]
    #[must_use]
    pub fn cp4(&mut self) -> CP4_W<NSACR_SPEC> {
        CP4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables Non-secure access to coprocessor CP5"]
    #[inline(always)]
    #[must_use]
    pub fn cp5(&mut self) -> CP5_W<NSACR_SPEC> {
        CP5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables Non-secure access to coprocessor CP6"]
    #[inline(always)]
    #[must_use]
    pub fn cp6(&mut self) -> CP6_W<NSACR_SPEC> {
        CP6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables Non-secure access to coprocessor CP7"]
    #[inline(always)]
    #[must_use]
    pub fn cp7(&mut self) -> CP7_W<NSACR_SPEC> {
        CP7_W::new(self, 7)
    }
    #[doc = "Bit 10 - Enables Non-secure access to the Floating-point Extension"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<NSACR_SPEC> {
        CP10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enables Non-secure access to the Floating-point Extension"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<NSACR_SPEC> {
        CP11_W::new(self, 11)
    }
}
#[doc = "Defines the Non-secure access permissions for both the FP Extension and coprocessors CP0 to CP7  

You can [`read`](crate::Reg::read) this register and get [`nsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSACR_SPEC;
impl crate::RegisterSpec for NSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsacr::R`](R) reader structure"]
impl crate::Readable for NSACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nsacr::W`](W) writer structure"]
impl crate::Writable for NSACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSACR to value 0"]
impl crate::Resettable for NSACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
