#[doc = "Register `CRITICAL` reader"]
pub type R = crate::R<CRITICAL_SPEC>;
#[doc = "Register `CRITICAL` writer"]
pub type W = crate::W<CRITICAL_SPEC>;
#[doc = "Field `SECURE_BOOT_ENABLE` reader - "]
pub type SECURE_BOOT_ENABLE_R = crate::BitReader;
#[doc = "Field `SECURE_DEBUG_DISABLE` reader - "]
pub type SECURE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `DEBUG_DISABLE` reader - "]
pub type DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `DEFAULT_ARCHSEL` reader - "]
pub type DEFAULT_ARCHSEL_R = crate::BitReader;
#[doc = "Field `GLITCH_DETECTOR_ENABLE` reader - "]
pub type GLITCH_DETECTOR_ENABLE_R = crate::BitReader;
#[doc = "Field `GLITCH_DETECTOR_SENS` reader - "]
pub type GLITCH_DETECTOR_SENS_R = crate::FieldReader;
#[doc = "Field `ARM_DISABLE` reader - "]
pub type ARM_DISABLE_R = crate::BitReader;
#[doc = "Field `RISCV_DISABLE` reader - "]
pub type RISCV_DISABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn secure_boot_enable(&self) -> SECURE_BOOT_ENABLE_R {
        SECURE_BOOT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn secure_debug_disable(&self) -> SECURE_DEBUG_DISABLE_R {
        SECURE_DEBUG_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn debug_disable(&self) -> DEBUG_DISABLE_R {
        DEBUG_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn default_archsel(&self) -> DEFAULT_ARCHSEL_R {
        DEFAULT_ARCHSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn glitch_detector_enable(&self) -> GLITCH_DETECTOR_ENABLE_R {
        GLITCH_DETECTOR_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn glitch_detector_sens(&self) -> GLITCH_DETECTOR_SENS_R {
        GLITCH_DETECTOR_SENS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn arm_disable(&self) -> ARM_DISABLE_R {
        ARM_DISABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn riscv_disable(&self) -> RISCV_DISABLE_R {
        RISCV_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {}
#[doc = "Quickly check values of critical flags read during boot up  

You can [`read`](crate::Reg::read) this register and get [`critical::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`critical::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRITICAL_SPEC;
impl crate::RegisterSpec for CRITICAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`critical::R`](R) reader structure"]
impl crate::Readable for CRITICAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`critical::W`](W) writer structure"]
impl crate::Writable for CRITICAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRITICAL to value 0"]
impl crate::Resettable for CRITICAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
