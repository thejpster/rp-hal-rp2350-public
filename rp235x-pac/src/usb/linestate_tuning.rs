#[doc = "Register `LINESTATE_TUNING` reader"]
pub type R = crate::R<LINESTATE_TUNING_SPEC>;
#[doc = "Register `LINESTATE_TUNING` writer"]
pub type W = crate::W<LINESTATE_TUNING_SPEC>;
#[doc = "Field `RCV_DELAY` reader - Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
pub type RCV_DELAY_R = crate::BitReader;
#[doc = "Field `RCV_DELAY` writer - Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
pub type RCV_DELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINESTATE_DELAY` reader - Device/Host - add an extra 1-bit debounce of linestate sampling."]
pub type LINESTATE_DELAY_R = crate::BitReader;
#[doc = "Field `LINESTATE_DELAY` writer - Device/Host - add an extra 1-bit debounce of linestate sampling."]
pub type LINESTATE_DELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTI_HUB_FIX` reader - Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
pub type MULTI_HUB_FIX_R = crate::BitReader;
#[doc = "Field `MULTI_HUB_FIX` writer - Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
pub type MULTI_HUB_FIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_BUFF_CONTROL_DOUBLE_READ_FIX` reader - Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
pub type DEV_BUFF_CONTROL_DOUBLE_READ_FIX_R = crate::BitReader;
#[doc = "Field `DEV_BUFF_CONTROL_DOUBLE_READ_FIX` writer - Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
pub type DEV_BUFF_CONTROL_DOUBLE_READ_FIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIE_RX_BITSTUFF_FIX` reader - RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
pub type SIE_RX_BITSTUFF_FIX_R = crate::BitReader;
#[doc = "Field `SIE_RX_BITSTUFF_FIX` writer - RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
pub type SIE_RX_BITSTUFF_FIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIE_RX_CHATTER_SE0_FIX` reader - RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
pub type SIE_RX_CHATTER_SE0_FIX_R = crate::BitReader;
#[doc = "Field `SIE_RX_CHATTER_SE0_FIX` writer - RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
pub type SIE_RX_CHATTER_SE0_FIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_RX_ERR_QUIESCE` reader - Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
pub type DEV_RX_ERR_QUIESCE_R = crate::BitReader;
#[doc = "Field `DEV_RX_ERR_QUIESCE` writer - Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
pub type DEV_RX_ERR_QUIESCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_LS_WAKE_FIX` reader - Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
pub type DEV_LS_WAKE_FIX_R = crate::BitReader;
#[doc = "Field `DEV_LS_WAKE_FIX` writer - Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
pub type DEV_LS_WAKE_FIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_FIX` reader - "]
pub type SPARE_FIX_R = crate::FieldReader;
#[doc = "Field `SPARE_FIX` writer - "]
pub type SPARE_FIX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
    #[inline(always)]
    pub fn rcv_delay(&self) -> RCV_DELAY_R {
        RCV_DELAY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device/Host - add an extra 1-bit debounce of linestate sampling."]
    #[inline(always)]
    pub fn linestate_delay(&self) -> LINESTATE_DELAY_R {
        LINESTATE_DELAY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
    #[inline(always)]
    pub fn multi_hub_fix(&self) -> MULTI_HUB_FIX_R {
        MULTI_HUB_FIX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
    #[inline(always)]
    pub fn dev_buff_control_double_read_fix(&self) -> DEV_BUFF_CONTROL_DOUBLE_READ_FIX_R {
        DEV_BUFF_CONTROL_DOUBLE_READ_FIX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
    #[inline(always)]
    pub fn sie_rx_bitstuff_fix(&self) -> SIE_RX_BITSTUFF_FIX_R {
        SIE_RX_BITSTUFF_FIX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
    #[inline(always)]
    pub fn sie_rx_chatter_se0_fix(&self) -> SIE_RX_CHATTER_SE0_FIX_R {
        SIE_RX_CHATTER_SE0_FIX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
    #[inline(always)]
    pub fn dev_rx_err_quiesce(&self) -> DEV_RX_ERR_QUIESCE_R {
        DEV_RX_ERR_QUIESCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
    #[inline(always)]
    pub fn dev_ls_wake_fix(&self) -> DEV_LS_WAKE_FIX_R {
        DEV_LS_WAKE_FIX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn spare_fix(&self) -> SPARE_FIX_R {
        SPARE_FIX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
    #[inline(always)]
    #[must_use]
    pub fn rcv_delay(&mut self) -> RCV_DELAY_W<LINESTATE_TUNING_SPEC> {
        RCV_DELAY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device/Host - add an extra 1-bit debounce of linestate sampling."]
    #[inline(always)]
    #[must_use]
    pub fn linestate_delay(&mut self) -> LINESTATE_DELAY_W<LINESTATE_TUNING_SPEC> {
        LINESTATE_DELAY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
    #[inline(always)]
    #[must_use]
    pub fn multi_hub_fix(&mut self) -> MULTI_HUB_FIX_W<LINESTATE_TUNING_SPEC> {
        MULTI_HUB_FIX_W::new(self, 2)
    }
    #[doc = "Bit 3 - Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
    #[inline(always)]
    #[must_use]
    pub fn dev_buff_control_double_read_fix(
        &mut self,
    ) -> DEV_BUFF_CONTROL_DOUBLE_READ_FIX_W<LINESTATE_TUNING_SPEC> {
        DEV_BUFF_CONTROL_DOUBLE_READ_FIX_W::new(self, 3)
    }
    #[doc = "Bit 4 - RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
    #[inline(always)]
    #[must_use]
    pub fn sie_rx_bitstuff_fix(&mut self) -> SIE_RX_BITSTUFF_FIX_W<LINESTATE_TUNING_SPEC> {
        SIE_RX_BITSTUFF_FIX_W::new(self, 4)
    }
    #[doc = "Bit 5 - RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
    #[inline(always)]
    #[must_use]
    pub fn sie_rx_chatter_se0_fix(&mut self) -> SIE_RX_CHATTER_SE0_FIX_W<LINESTATE_TUNING_SPEC> {
        SIE_RX_CHATTER_SE0_FIX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
    #[inline(always)]
    #[must_use]
    pub fn dev_rx_err_quiesce(&mut self) -> DEV_RX_ERR_QUIESCE_W<LINESTATE_TUNING_SPEC> {
        DEV_RX_ERR_QUIESCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
    #[inline(always)]
    #[must_use]
    pub fn dev_ls_wake_fix(&mut self) -> DEV_LS_WAKE_FIX_W<LINESTATE_TUNING_SPEC> {
        DEV_LS_WAKE_FIX_W::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn spare_fix(&mut self) -> SPARE_FIX_W<LINESTATE_TUNING_SPEC> {
        SPARE_FIX_W::new(self, 8)
    }
}
#[doc = "Used for debug only.  

You can [`read`](crate::Reg::read) this register and get [`linestate_tuning::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linestate_tuning::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINESTATE_TUNING_SPEC;
impl crate::RegisterSpec for LINESTATE_TUNING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linestate_tuning::R`](R) reader structure"]
impl crate::Readable for LINESTATE_TUNING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linestate_tuning::W`](W) writer structure"]
impl crate::Writable for LINESTATE_TUNING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINESTATE_TUNING to value 0xf8"]
impl crate::Resettable for LINESTATE_TUNING_SPEC {
    const RESET_VALUE: u32 = 0xf8;
}
