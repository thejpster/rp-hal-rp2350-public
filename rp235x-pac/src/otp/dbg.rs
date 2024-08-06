#[doc = "Register `DBG` reader"]
pub type R = crate::R<DBG_SPEC>;
#[doc = "Register `DBG` writer"]
pub type W = crate::W<DBG_SPEC>;
#[doc = "Field `PSM_DONE` reader - PSM done status flag"]
pub type PSM_DONE_R = crate::BitReader;
#[doc = "Field `BOOT_DONE` reader - PSM boot done status flag"]
pub type BOOT_DONE_R = crate::BitReader;
#[doc = "Field `ROSC_UP_SEEN` reader - Ring oscillator was seen up and running"]
pub type ROSC_UP_SEEN_R = crate::BitReader;
#[doc = "Field `ROSC_UP_SEEN` writer - Ring oscillator was seen up and running"]
pub type ROSC_UP_SEEN_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ROSC_UP` reader - Ring oscillator is up and running"]
pub type ROSC_UP_R = crate::BitReader;
#[doc = "Field `PSM_STATE` reader - Monitor the PSM FSM's state"]
pub type PSM_STATE_R = crate::FieldReader;
#[doc = "Field `CUSTOMER_RMA_FLAG` reader - The chip is in RMA mode"]
pub type CUSTOMER_RMA_FLAG_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PSM done status flag"]
    #[inline(always)]
    pub fn psm_done(&self) -> PSM_DONE_R {
        PSM_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSM boot done status flag"]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ring oscillator was seen up and running"]
    #[inline(always)]
    pub fn rosc_up_seen(&self) -> ROSC_UP_SEEN_R {
        ROSC_UP_SEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ring oscillator is up and running"]
    #[inline(always)]
    pub fn rosc_up(&self) -> ROSC_UP_R {
        ROSC_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Monitor the PSM FSM's state"]
    #[inline(always)]
    pub fn psm_state(&self) -> PSM_STATE_R {
        PSM_STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - The chip is in RMA mode"]
    #[inline(always)]
    pub fn customer_rma_flag(&self) -> CUSTOMER_RMA_FLAG_R {
        CUSTOMER_RMA_FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Ring oscillator was seen up and running"]
    #[inline(always)]
    #[must_use]
    pub fn rosc_up_seen(&mut self) -> ROSC_UP_SEEN_W<DBG_SPEC> {
        ROSC_UP_SEEN_W::new(self, 2)
    }
}
#[doc = "Debug for OTP power-on state machine  

You can [`read`](crate::Reg::read) this register and get [`dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_SPEC;
impl crate::RegisterSpec for DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg::R`](R) reader structure"]
impl crate::Readable for DBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg::W`](W) writer structure"]
impl crate::Writable for DBG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets DBG to value 0"]
impl crate::Resettable for DBG_SPEC {
    const RESET_VALUE: u32 = 0;
}
