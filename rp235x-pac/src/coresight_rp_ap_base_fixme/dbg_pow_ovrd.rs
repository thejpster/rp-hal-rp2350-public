#[doc = "Register `DBG_POW_OVRD` reader"]
pub type R = crate::R<DBG_POW_OVRD_SPEC>;
#[doc = "Register `DBG_POW_OVRD` writer"]
pub type W = crate::W<DBG_POW_OVRD_SPEC>;
#[doc = "Field `DBG_POW_OVRD_SMALL_REQ` reader - Turn on the small power switches for all domains. This switches on chain 0 for each domain and switches off chains 2 &amp; 3 and the large power switch chain. This will bring the power up for all domains without browning out the always-on power domain."]
pub type DBG_POW_OVRD_SMALL_REQ_R = crate::BitReader;
#[doc = "Field `DBG_POW_OVRD_SMALL_REQ` writer - Turn on the small power switches for all domains. This switches on chain 0 for each domain and switches off chains 2 &amp; 3 and the large power switch chain. This will bring the power up for all domains without browning out the always-on power domain."]
pub type DBG_POW_OVRD_SMALL_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_OVRD_LARGE_REQ` reader - Turn on the large power switches for all domains. This should not be done until sufficient time has been allowed for the small switches to bring the supplies up. Switching the large switches on too soon risks browning out the always-on domain and corrupting these very registers."]
pub type DBG_POW_OVRD_LARGE_REQ_R = crate::BitReader;
#[doc = "Field `DBG_POW_OVRD_LARGE_REQ` writer - Turn on the large power switches for all domains. This should not be done until sufficient time has been allowed for the small switches to bring the supplies up. Switching the large switches on too soon risks browning out the always-on domain and corrupting these very registers."]
pub type DBG_POW_OVRD_LARGE_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_OVRD_ISO` reader - Enables DBG_POW_ISO to control the isolation gates between domains."]
pub type DBG_POW_OVRD_ISO_R = crate::BitReader;
#[doc = "Field `DBG_POW_OVRD_ISO` writer - Enables DBG_POW_ISO to control the isolation gates between domains."]
pub type DBG_POW_OVRD_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_ISO` reader - When DBG_POW_OVRD_ISO=1 this register bit controls the isolation gates for all domains. 1 = isolated. 0 = not isolated."]
pub type DBG_POW_ISO_R = crate::BitReader;
#[doc = "Field `DBG_POW_ISO` writer - When DBG_POW_OVRD_ISO=1 this register bit controls the isolation gates for all domains. 1 = isolated. 0 = not isolated."]
pub type DBG_POW_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_OVRD_RESET` reader - Enables DBG_POW_RESET to control the resets for the power manager and the switched-core. Essentially that is everythjing except the Coresight 2-wire interface and the RP_AP registers."]
pub type DBG_POW_OVRD_RESET_R = crate::BitReader;
#[doc = "Field `DBG_POW_OVRD_RESET` writer - Enables DBG_POW_RESET to control the resets for the power manager and the switched-core. Essentially that is everythjing except the Coresight 2-wire interface and the RP_AP registers."]
pub type DBG_POW_OVRD_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_RESET` reader - When DBG_POW_OVRD_RESET=1 this register bit controls the resets for all domains. 1 = reset. 0 = not reset."]
pub type DBG_POW_RESET_R = crate::BitReader;
#[doc = "Field `DBG_POW_RESET` writer - When DBG_POW_OVRD_RESET=1 this register bit controls the resets for all domains. 1 = reset. 0 = not reset."]
pub type DBG_POW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_POW_RESTART_FROM_XOSC` reader - By default the system begins boot as soon as a clock is available from the ROSC, then it switches to the XOSC when it is available. This is done because the XOSC takes several ms to start up. If there is a problem with the ROSC then the default behaviour can be changed to not use the ROSC and wait for XOSC. However, this requires a mask change to modify the reset value of the Power Manager START_FROM_XOSC register. To allow experimentation the default can be temporarily changed by setting this register bit to 1. After setting this bit the core must be reset by a Coresight dprst or a rescue reset (see RESCUE_RESTART in the RP_AP_CTRL register above). A power-on reset, brown-out reset or RUN pin reset will reset this control and revert to the default behaviour."]
pub type DBG_POW_RESTART_FROM_XOSC_R = crate::BitReader;
#[doc = "Field `DBG_POW_RESTART_FROM_XOSC` writer - By default the system begins boot as soon as a clock is available from the ROSC, then it switches to the XOSC when it is available. This is done because the XOSC takes several ms to start up. If there is a problem with the ROSC then the default behaviour can be changed to not use the ROSC and wait for XOSC. However, this requires a mask change to modify the reset value of the Power Manager START_FROM_XOSC register. To allow experimentation the default can be temporarily changed by setting this register bit to 1. After setting this bit the core must be reset by a Coresight dprst or a rescue reset (see RESCUE_RESTART in the RP_AP_CTRL register above). A power-on reset, brown-out reset or RUN pin reset will reset this control and revert to the default behaviour."]
pub type DBG_POW_RESTART_FROM_XOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Turn on the small power switches for all domains. This switches on chain 0 for each domain and switches off chains 2 &amp; 3 and the large power switch chain. This will bring the power up for all domains without browning out the always-on power domain."]
    #[inline(always)]
    pub fn dbg_pow_ovrd_small_req(&self) -> DBG_POW_OVRD_SMALL_REQ_R {
        DBG_POW_OVRD_SMALL_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Turn on the large power switches for all domains. This should not be done until sufficient time has been allowed for the small switches to bring the supplies up. Switching the large switches on too soon risks browning out the always-on domain and corrupting these very registers."]
    #[inline(always)]
    pub fn dbg_pow_ovrd_large_req(&self) -> DBG_POW_OVRD_LARGE_REQ_R {
        DBG_POW_OVRD_LARGE_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables DBG_POW_ISO to control the isolation gates between domains."]
    #[inline(always)]
    pub fn dbg_pow_ovrd_iso(&self) -> DBG_POW_OVRD_ISO_R {
        DBG_POW_OVRD_ISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When DBG_POW_OVRD_ISO=1 this register bit controls the isolation gates for all domains. 1 = isolated. 0 = not isolated."]
    #[inline(always)]
    pub fn dbg_pow_iso(&self) -> DBG_POW_ISO_R {
        DBG_POW_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables DBG_POW_RESET to control the resets for the power manager and the switched-core. Essentially that is everythjing except the Coresight 2-wire interface and the RP_AP registers."]
    #[inline(always)]
    pub fn dbg_pow_ovrd_reset(&self) -> DBG_POW_OVRD_RESET_R {
        DBG_POW_OVRD_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When DBG_POW_OVRD_RESET=1 this register bit controls the resets for all domains. 1 = reset. 0 = not reset."]
    #[inline(always)]
    pub fn dbg_pow_reset(&self) -> DBG_POW_RESET_R {
        DBG_POW_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - By default the system begins boot as soon as a clock is available from the ROSC, then it switches to the XOSC when it is available. This is done because the XOSC takes several ms to start up. If there is a problem with the ROSC then the default behaviour can be changed to not use the ROSC and wait for XOSC. However, this requires a mask change to modify the reset value of the Power Manager START_FROM_XOSC register. To allow experimentation the default can be temporarily changed by setting this register bit to 1. After setting this bit the core must be reset by a Coresight dprst or a rescue reset (see RESCUE_RESTART in the RP_AP_CTRL register above). A power-on reset, brown-out reset or RUN pin reset will reset this control and revert to the default behaviour."]
    #[inline(always)]
    pub fn dbg_pow_restart_from_xosc(&self) -> DBG_POW_RESTART_FROM_XOSC_R {
        DBG_POW_RESTART_FROM_XOSC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Turn on the small power switches for all domains. This switches on chain 0 for each domain and switches off chains 2 &amp; 3 and the large power switch chain. This will bring the power up for all domains without browning out the always-on power domain."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_ovrd_small_req(&mut self) -> DBG_POW_OVRD_SMALL_REQ_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_OVRD_SMALL_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Turn on the large power switches for all domains. This should not be done until sufficient time has been allowed for the small switches to bring the supplies up. Switching the large switches on too soon risks browning out the always-on domain and corrupting these very registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_ovrd_large_req(&mut self) -> DBG_POW_OVRD_LARGE_REQ_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_OVRD_LARGE_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables DBG_POW_ISO to control the isolation gates between domains."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_ovrd_iso(&mut self) -> DBG_POW_OVRD_ISO_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_OVRD_ISO_W::new(self, 2)
    }
    #[doc = "Bit 3 - When DBG_POW_OVRD_ISO=1 this register bit controls the isolation gates for all domains. 1 = isolated. 0 = not isolated."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_iso(&mut self) -> DBG_POW_ISO_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_ISO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables DBG_POW_RESET to control the resets for the power manager and the switched-core. Essentially that is everythjing except the Coresight 2-wire interface and the RP_AP registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_ovrd_reset(&mut self) -> DBG_POW_OVRD_RESET_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_OVRD_RESET_W::new(self, 4)
    }
    #[doc = "Bit 5 - When DBG_POW_OVRD_RESET=1 this register bit controls the resets for all domains. 1 = reset. 0 = not reset."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_reset(&mut self) -> DBG_POW_RESET_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_RESET_W::new(self, 5)
    }
    #[doc = "Bit 6 - By default the system begins boot as soon as a clock is available from the ROSC, then it switches to the XOSC when it is available. This is done because the XOSC takes several ms to start up. If there is a problem with the ROSC then the default behaviour can be changed to not use the ROSC and wait for XOSC. However, this requires a mask change to modify the reset value of the Power Manager START_FROM_XOSC register. To allow experimentation the default can be temporarily changed by setting this register bit to 1. After setting this bit the core must be reset by a Coresight dprst or a rescue reset (see RESCUE_RESTART in the RP_AP_CTRL register above). A power-on reset, brown-out reset or RUN pin reset will reset this control and revert to the default behaviour."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pow_restart_from_xosc(&mut self) -> DBG_POW_RESTART_FROM_XOSC_W<DBG_POW_OVRD_SPEC> {
        DBG_POW_RESTART_FROM_XOSC_W::new(self, 6)
    }
}
#[doc = "This register allows external control of the power sequencer outputs for all the switched power domains. If any of the power sequencers stall at any stage then force power up operation of all domains by running this sequence:  
 - set DBG_POW_OVRD = 0x3b to force small power switches on, large power switches off, resets on and isolation on  
 - allow time for the domain power supplies to reach full rail  
 - set DBG_POW_OVRD = 0x3b to force large power switches on  
 - set DBG_POW_OVRD = 0x37 to remove isolation  
 - set DBG_POW_OVRD = 0x17 to remove resets  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_ovrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pow_ovrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_POW_OVRD_SPEC;
impl crate::RegisterSpec for DBG_POW_OVRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_pow_ovrd::R`](R) reader structure"]
impl crate::Readable for DBG_POW_OVRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_pow_ovrd::W`](W) writer structure"]
impl crate::Writable for DBG_POW_OVRD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_POW_OVRD to value 0"]
impl crate::Resettable for DBG_POW_OVRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
