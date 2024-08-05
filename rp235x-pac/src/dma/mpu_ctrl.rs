#[doc = "Register `MPU_CTRL` reader"]
pub type R = crate::R<MPU_CTRL_SPEC>;
#[doc = "Register `MPU_CTRL` writer"]
pub type W = crate::W<MPU_CTRL_SPEC>;
#[doc = "Field `P` reader - Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS_HIDE_ADDR` reader - By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
pub type NS_HIDE_ADDR_R = crate::BitReader;
#[doc = "Field `NS_HIDE_ADDR` writer - By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
pub type NS_HIDE_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
    #[inline(always)]
    pub fn ns_hide_addr(&self) -> NS_HIDE_ADDR_R {
        NS_HIDE_ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<MPU_CTRL_SPEC> {
        P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<MPU_CTRL_SPEC> {
        S_W::new(self, 2)
    }
    #[doc = "Bit 3 - By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
    #[inline(always)]
    #[must_use]
    pub fn ns_hide_addr(&mut self) -> NS_HIDE_ADDR_W<MPU_CTRL_SPEC> {
        NS_HIDE_ADDR_W::new(self, 3)
    }
}
#[doc = "Control register for DMA MPU. Accessible only from a Privileged context.  

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
