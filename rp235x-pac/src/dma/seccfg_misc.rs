#[doc = "Register `SECCFG_MISC` reader"]
pub type R = crate::R<SECCFG_MISC_SPEC>;
#[doc = "Register `SECCFG_MISC` writer"]
pub type W = crate::W<SECCFG_MISC_SPEC>;
#[doc = "Field `SNIFF_P` reader - If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
pub type SNIFF_P_R = crate::BitReader;
#[doc = "Field `SNIFF_P` writer - If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
pub type SNIFF_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNIFF_S` reader - If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
pub type SNIFF_S_R = crate::BitReader;
#[doc = "Field `SNIFF_S` writer - If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
pub type SNIFF_S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_P` reader - If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
pub type TIMER0_P_R = crate::BitReader;
#[doc = "Field `TIMER0_P` writer - If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
pub type TIMER0_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_S` reader - If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
pub type TIMER0_S_R = crate::BitReader;
#[doc = "Field `TIMER0_S` writer - If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
pub type TIMER0_S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_P` reader - If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
pub type TIMER1_P_R = crate::BitReader;
#[doc = "Field `TIMER1_P` writer - If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
pub type TIMER1_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_S` reader - If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
pub type TIMER1_S_R = crate::BitReader;
#[doc = "Field `TIMER1_S` writer - If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
pub type TIMER1_S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_P` reader - If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
pub type TIMER2_P_R = crate::BitReader;
#[doc = "Field `TIMER2_P` writer - If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
pub type TIMER2_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_S` reader - If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
pub type TIMER2_S_R = crate::BitReader;
#[doc = "Field `TIMER2_S` writer - If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
pub type TIMER2_S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_P` reader - If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
pub type TIMER3_P_R = crate::BitReader;
#[doc = "Field `TIMER3_P` writer - If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
pub type TIMER3_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_S` reader - If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
pub type TIMER3_S_R = crate::BitReader;
#[doc = "Field `TIMER3_S` writer - If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
pub type TIMER3_S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
    #[inline(always)]
    pub fn sniff_p(&self) -> SNIFF_P_R {
        SNIFF_P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
    #[inline(always)]
    pub fn sniff_s(&self) -> SNIFF_S_R {
        SNIFF_S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn timer0_p(&self) -> TIMER0_P_R {
        TIMER0_P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
    #[inline(always)]
    pub fn timer0_s(&self) -> TIMER0_S_R {
        TIMER0_S_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn timer1_p(&self) -> TIMER1_P_R {
        TIMER1_P_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
    #[inline(always)]
    pub fn timer1_s(&self) -> TIMER1_S_R {
        TIMER1_S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn timer2_p(&self) -> TIMER2_P_R {
        TIMER2_P_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
    #[inline(always)]
    pub fn timer2_s(&self) -> TIMER2_S_R {
        TIMER2_S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn timer3_p(&self) -> TIMER3_P_R {
        TIMER3_P_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
    #[inline(always)]
    pub fn timer3_s(&self) -> TIMER3_S_R {
        TIMER3_S_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
    #[inline(always)]
    #[must_use]
    pub fn sniff_p(&mut self) -> SNIFF_P_W<SECCFG_MISC_SPEC> {
        SNIFF_P_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn sniff_s(&mut self) -> SNIFF_S_W<SECCFG_MISC_SPEC> {
        SNIFF_S_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_p(&mut self) -> TIMER0_P_W<SECCFG_MISC_SPEC> {
        TIMER0_P_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_s(&mut self) -> TIMER0_S_W<SECCFG_MISC_SPEC> {
        TIMER0_S_W::new(self, 3)
    }
    #[doc = "Bit 4 - If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_p(&mut self) -> TIMER1_P_W<SECCFG_MISC_SPEC> {
        TIMER1_P_W::new(self, 4)
    }
    #[doc = "Bit 5 - If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_s(&mut self) -> TIMER1_S_W<SECCFG_MISC_SPEC> {
        TIMER1_S_W::new(self, 5)
    }
    #[doc = "Bit 6 - If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_p(&mut self) -> TIMER2_P_W<SECCFG_MISC_SPEC> {
        TIMER2_P_W::new(self, 6)
    }
    #[doc = "Bit 7 - If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_s(&mut self) -> TIMER2_S_W<SECCFG_MISC_SPEC> {
        TIMER2_S_W::new(self, 7)
    }
    #[doc = "Bit 8 - If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_p(&mut self) -> TIMER3_P_W<SECCFG_MISC_SPEC> {
        TIMER3_P_W::new(self, 8)
    }
    #[doc = "Bit 9 - If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_s(&mut self) -> TIMER3_S_W<SECCFG_MISC_SPEC> {
        TIMER3_S_W::new(self, 9)
    }
}
#[doc = "Miscellaneous security configuration  

You can [`read`](crate::Reg::read) this register and get [`seccfg_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFG_MISC_SPEC;
impl crate::RegisterSpec for SECCFG_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfg_misc::R`](R) reader structure"]
impl crate::Readable for SECCFG_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seccfg_misc::W`](W) writer structure"]
impl crate::Writable for SECCFG_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFG_MISC to value 0x03ff"]
impl crate::Resettable for SECCFG_MISC_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
