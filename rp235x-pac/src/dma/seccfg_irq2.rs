#[doc = "Register `SECCFG_IRQ2` reader"]
pub type R = crate::R<SECCFG_IRQ2_SPEC>;
#[doc = "Register `SECCFG_IRQ2` writer"]
pub type W = crate::W<SECCFG_IRQ2_SPEC>;
#[doc = "Field `P` reader - Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<SECCFG_IRQ2_SPEC> {
        P_W::new(self, 0)
    }
    #[doc = "Bit 1 - Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SECCFG_IRQ2_SPEC> {
        S_W::new(self, 1)
    }
}
#[doc = "Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_irq2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_irq2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFG_IRQ2_SPEC;
impl crate::RegisterSpec for SECCFG_IRQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfg_irq2::R`](R) reader structure"]
impl crate::Readable for SECCFG_IRQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seccfg_irq2::W`](W) writer structure"]
impl crate::Writable for SECCFG_IRQ2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFG_IRQ2 to value 0x03"]
impl crate::Resettable for SECCFG_IRQ2_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
