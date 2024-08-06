#[doc = "Register `VREG_CTRL` reader"]
pub type R = crate::R<VREG_CTRL_SPEC>;
#[doc = "Register `VREG_CTRL` writer"]
pub type W = crate::W<VREG_CTRL_SPEC>;
#[doc = "Field `HT_TH` reader - high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
pub type HT_TH_R = crate::FieldReader;
#[doc = "Field `HT_TH` writer - high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
pub type HT_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISABLE_VOLTAGE_LIMIT` reader - 0=not disabled, 1=enabled"]
pub type DISABLE_VOLTAGE_LIMIT_R = crate::BitReader;
#[doc = "Field `DISABLE_VOLTAGE_LIMIT` writer - 0=not disabled, 1=enabled"]
pub type DISABLE_VOLTAGE_LIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOLATE` reader - isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
pub type ISOLATE_R = crate::BitReader;
#[doc = "Field `ISOLATE` writer - isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
pub type ISOLATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` reader - unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
pub type UNLOCK_R = crate::BitReader;
#[doc = "Field `UNLOCK` writer - unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_N` reader - returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
pub type RST_N_R = crate::BitReader;
#[doc = "Field `RST_N` writer - returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
pub type RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:6 - high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
    #[inline(always)]
    pub fn ht_th(&self) -> HT_TH_R {
        HT_TH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - 0=not disabled, 1=enabled"]
    #[inline(always)]
    pub fn disable_voltage_limit(&self) -> DISABLE_VOLTAGE_LIMIT_R {
        DISABLE_VOLTAGE_LIMIT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub fn isolate(&self) -> ISOLATE_R {
        ISOLATE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
    #[inline(always)]
    pub fn rst_n(&self) -> RST_N_R {
        RST_N_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
    #[inline(always)]
    #[must_use]
    pub fn ht_th(&mut self) -> HT_TH_W<VREG_CTRL_SPEC> {
        HT_TH_W::new(self, 4)
    }
    #[doc = "Bit 8 - 0=not disabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn disable_voltage_limit(&mut self) -> DISABLE_VOLTAGE_LIMIT_W<VREG_CTRL_SPEC> {
        DISABLE_VOLTAGE_LIMIT_W::new(self, 8)
    }
    #[doc = "Bit 12 - isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    #[must_use]
    pub fn isolate(&mut self) -> ISOLATE_W<VREG_CTRL_SPEC> {
        ISOLATE_W::new(self, 12)
    }
    #[doc = "Bit 13 - unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<VREG_CTRL_SPEC> {
        UNLOCK_W::new(self, 13)
    }
    #[doc = "Bit 15 - returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n(&mut self) -> RST_N_W<VREG_CTRL_SPEC> {
        RST_N_W::new(self, 15)
    }
}
#[doc = "Voltage Regulator Control  

You can [`read`](crate::Reg::read) this register and get [`vreg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_CTRL_SPEC;
impl crate::RegisterSpec for VREG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg_ctrl::R`](R) reader structure"]
impl crate::Readable for VREG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg_ctrl::W`](W) writer structure"]
impl crate::Writable for VREG_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREG_CTRL to value 0x8050"]
impl crate::Resettable for VREG_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x8050;
}
