#[doc = "Register `MPU_BAR2` reader"]
pub type R = crate::R<MPU_BAR2_SPEC>;
#[doc = "Register `MPU_BAR2` writer"]
pub type W = crate::W<MPU_BAR2_SPEC>;
#[doc = "Field `ADDR` reader - This MPU region matches addresses where addr\\[31:5\\]
(the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - This MPU region matches addresses where addr\\[31:5\\]
(the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - This MPU region matches addresses where addr\\[31:5\\]
(the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - This MPU region matches addresses where addr\\[31:5\\]
(the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<MPU_BAR2_SPEC> {
        ADDR_W::new(self, 5)
    }
}
#[doc = "Base address register for MPU region 2. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_BAR2_SPEC;
impl crate::RegisterSpec for MPU_BAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_bar2::R`](R) reader structure"]
impl crate::Readable for MPU_BAR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_bar2::W`](W) writer structure"]
impl crate::Writable for MPU_BAR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_BAR2 to value 0"]
impl crate::Resettable for MPU_BAR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
