#[doc = "Register `MPU_CTRL` reader"]
pub type R = crate::R<MPU_CTRL_SPEC>;
#[doc = "Register `MPU_CTRL` writer"]
pub type W = crate::W<MPU_CTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Enables the MPU"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the MPU"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFNMIENA` reader - Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub type HFNMIENA_R = crate::BitReader;
#[doc = "Field `HFNMIENA` writer - Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub type HFNMIENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIVDEFENA` reader - Controls whether the default memory map is enabled for privileged software"]
pub type PRIVDEFENA_R = crate::BitReader;
#[doc = "Field `PRIVDEFENA` writer - Controls whether the default memory map is enabled for privileged software"]
pub type PRIVDEFENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the MPU"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls whether the default memory map is enabled for privileged software"]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MPU"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MPU_CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or disabled. This applies to HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W<MPU_CTRL_SPEC> {
        HFNMIENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Controls whether the default memory map is enabled for privileged software"]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W<MPU_CTRL_SPEC> {
        PRIVDEFENA_W::new(self, 2)
    }
}
#[doc = "Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1  

You can [`read`](crate::Reg::read) this register and get [`mpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_ctrl::R`](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_ctrl::W`](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
