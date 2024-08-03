#[doc = "Register `MPU_RBAR_A2` reader"]
pub type R = crate::R<MPU_RBAR_A2_SPEC>;
#[doc = "Register `MPU_RBAR_A2` writer"]
pub type W = crate::W<MPU_RBAR_A2_SPEC>;
#[doc = "Field `XN` reader - Defines whether code can be executed from this region"]
pub type XN_R = crate::BitReader;
#[doc = "Field `XN` writer - Defines whether code can be executed from this region"]
pub type XN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AP` reader - Defines the access permissions for this region"]
pub type AP_R = crate::FieldReader;
#[doc = "Field `AP` writer - Defines the access permissions for this region"]
pub type AP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SH` reader - Defines the Shareability domain of this region for Normal memory"]
pub type SH_R = crate::FieldReader;
#[doc = "Field `SH` writer - Defines the Shareability domain of this region for Normal memory"]
pub type SH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BASE` reader - Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BASE_R = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - Defines whether code can be executed from this region"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Defines the access permissions for this region"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    pub fn sh(&self) -> SH_R {
        SH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Defines whether code can be executed from this region"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XN_W<MPU_RBAR_A2_SPEC> {
        XN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Defines the access permissions for this region"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<MPU_RBAR_A2_SPEC> {
        AP_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> SH_W<MPU_RBAR_A2_SPEC> {
        SH_W::new(self, 3)
    }
    #[doc = "Bits 5:31 - Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<MPU_RBAR_A2_SPEC> {
        BASE_W::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rbar_a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RBAR_A2_SPEC;
impl crate::RegisterSpec for MPU_RBAR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar_a2::R`](R) reader structure"]
impl crate::Readable for MPU_RBAR_A2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar_a2::W`](W) writer structure"]
impl crate::Writable for MPU_RBAR_A2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR_A2 to value 0"]
impl crate::Resettable for MPU_RBAR_A2_SPEC {
    const RESET_VALUE: u32 = 0;
}
