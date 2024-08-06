#[doc = "Register `MPU_LAR1` reader"]
pub type R = crate::R<MPU_LAR1_SPEC>;
#[doc = "Register `MPU_LAR1` writer"]
pub type W = crate::W<MPU_LAR1_SPEC>;
#[doc = "Field `EN` reader - Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P` reader - Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:31 - Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MPU_LAR1_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<MPU_LAR1_SPEC> {
        P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<MPU_LAR1_SPEC> {
        S_W::new(self, 2)
    }
    #[doc = "Bits 5:31 - Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<MPU_LAR1_SPEC> {
        ADDR_W::new(self, 5)
    }
}
#[doc = "Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_LAR1_SPEC;
impl crate::RegisterSpec for MPU_LAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_lar1::R`](R) reader structure"]
impl crate::Readable for MPU_LAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_lar1::W`](W) writer structure"]
impl crate::Writable for MPU_LAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_LAR1 to value 0"]
impl crate::Resettable for MPU_LAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
