#[doc = "Register `SECCFG_CH14` reader"]
pub type R = crate::R<SECCFG_CH14_SPEC>;
#[doc = "Register `SECCFG_CH14` writer"]
pub type W = crate::W<SECCFG_CH14_SPEC>;
#[doc = "Field `P` reader - Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<SECCFG_CH14_SPEC> {
        P_W::new(self, 0)
    }
    #[doc = "Bit 1 - Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SECCFG_CH14_SPEC> {
        S_W::new(self, 1)
    }
    #[doc = "Bit 2 - LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<SECCFG_CH14_SPEC> {
        LOCK_W::new(self, 2)
    }
}
#[doc = "Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFG_CH14_SPEC;
impl crate::RegisterSpec for SECCFG_CH14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfg_ch14::R`](R) reader structure"]
impl crate::Readable for SECCFG_CH14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seccfg_ch14::W`](W) writer structure"]
impl crate::Writable for SECCFG_CH14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFG_CH14 to value 0x03"]
impl crate::Resettable for SECCFG_CH14_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
