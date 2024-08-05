#[doc = "Register `MPU_TYPE` reader"]
pub type R = crate::R<MPU_TYPE_SPEC>;
#[doc = "Register `MPU_TYPE` writer"]
pub type W = crate::W<MPU_TYPE_SPEC>;
#[doc = "Field `SEPARATE` reader - Indicates support for separate instructions and data address regions"]
pub type SEPARATE_R = crate::BitReader;
#[doc = "Field `DREGION` reader - Number of regions supported by the MPU"]
pub type DREGION_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates support for separate instructions and data address regions"]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of regions supported by the MPU"]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "The MPU Type Register indicates how many regions the MPU `FTSSS supports  

You can [`read`](crate::Reg::read) this register and get [`mpu_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_TYPE_SPEC;
impl crate::RegisterSpec for MPU_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_type::R`](R) reader structure"]
impl crate::Readable for MPU_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_type::W`](W) writer structure"]
impl crate::Writable for MPU_TYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_TYPE to value 0x0800"]
impl crate::Resettable for MPU_TYPE_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
