#[doc = "Register `CRIT1` reader"]
pub type R = crate::R<CRIT1_SPEC>;
#[doc = "Register `CRIT1` writer"]
pub type W = crate::W<CRIT1_SPEC>;
#[doc = "Field `SECURE_BOOT_ENABLE` reader - Enable boot signature enforcement, and permanently disable the RISC-V cores."]
pub type SECURE_BOOT_ENABLE_R = crate::BitReader;
#[doc = "Field `SECURE_DEBUG_DISABLE` reader - Disable Secure debug access"]
pub type SECURE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `DEBUG_DISABLE` reader - Disable all debug access"]
pub type DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `BOOT_ARCH` reader - Set the default boot architecture, 0=ARM 1=RISC-V. Ignored if ARM_DISABLE, RISCV_DISABLE or SECURE_BOOT_ENABLE is set."]
pub type BOOT_ARCH_R = crate::BitReader;
#[doc = "Field `GLITCH_DETECTOR_ENABLE` reader - Arm the glitch detectors to reset the system if an abnormal clock/power event is observed."]
pub type GLITCH_DETECTOR_ENABLE_R = crate::BitReader;
#[doc = "Field `GLITCH_DETECTOR_SENS` reader - Increase the sensitivity of the glitch detectors from their default."]
pub type GLITCH_DETECTOR_SENS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable boot signature enforcement, and permanently disable the RISC-V cores."]
    #[inline(always)]
    pub fn secure_boot_enable(&self) -> SECURE_BOOT_ENABLE_R {
        SECURE_BOOT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Secure debug access"]
    #[inline(always)]
    pub fn secure_debug_disable(&self) -> SECURE_DEBUG_DISABLE_R {
        SECURE_DEBUG_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable all debug access"]
    #[inline(always)]
    pub fn debug_disable(&self) -> DEBUG_DISABLE_R {
        DEBUG_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set the default boot architecture, 0=ARM 1=RISC-V. Ignored if ARM_DISABLE, RISCV_DISABLE or SECURE_BOOT_ENABLE is set."]
    #[inline(always)]
    pub fn boot_arch(&self) -> BOOT_ARCH_R {
        BOOT_ARCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arm the glitch detectors to reset the system if an abnormal clock/power event is observed."]
    #[inline(always)]
    pub fn glitch_detector_enable(&self) -> GLITCH_DETECTOR_ENABLE_R {
        GLITCH_DETECTOR_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Increase the sensitivity of the glitch detectors from their default."]
    #[inline(always)]
    pub fn glitch_detector_sens(&self) -> GLITCH_DETECTOR_SENS_R {
        GLITCH_DETECTOR_SENS_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {}
#[doc = "Page 1 critical boot flags (RBIT-8)  

You can [`read`](crate::Reg::read) this register and get [`crit1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crit1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT1_SPEC;
impl crate::RegisterSpec for CRIT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit1::R`](R) reader structure"]
impl crate::Readable for CRIT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crit1::W`](W) writer structure"]
impl crate::Writable for CRIT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRIT1 to value 0"]
impl crate::Resettable for CRIT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
