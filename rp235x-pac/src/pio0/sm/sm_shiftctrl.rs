#[doc = "Register `SM_SHIFTCTRL` reader"]
pub type R = crate::R<SM_SHIFTCTRL_SPEC>;
#[doc = "Register `SM_SHIFTCTRL` writer"]
pub type W = crate::W<SM_SHIFTCTRL_SPEC>;
#[doc = "Field `IN_COUNT` reader - Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
pub type IN_COUNT_R = crate::FieldReader;
#[doc = "Field `IN_COUNT` writer - Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
pub type IN_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FJOIN_RX_GET` reader - If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
pub type FJOIN_RX_GET_R = crate::BitReader;
#[doc = "Field `FJOIN_RX_GET` writer - If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
pub type FJOIN_RX_GET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FJOIN_RX_PUT` reader - If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
pub type FJOIN_RX_PUT_R = crate::BitReader;
#[doc = "Field `FJOIN_RX_PUT` writer - If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
pub type FJOIN_RX_PUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOPUSH` reader - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
pub type AUTOPUSH_R = crate::BitReader;
#[doc = "Field `AUTOPUSH` writer - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
pub type AUTOPUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOPULL` reader - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
pub type AUTOPULL_R = crate::BitReader;
#[doc = "Field `AUTOPULL` writer - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
pub type AUTOPULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SHIFTDIR` reader - 1 = shift input shift register to right (data enters from left). 0 = to left."]
pub type IN_SHIFTDIR_R = crate::BitReader;
#[doc = "Field `IN_SHIFTDIR` writer - 1 = shift input shift register to right (data enters from left). 0 = to left."]
pub type IN_SHIFTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_SHIFTDIR` reader - 1 = shift out of output shift register to right. 0 = to left."]
pub type OUT_SHIFTDIR_R = crate::BitReader;
#[doc = "Field `OUT_SHIFTDIR` writer - 1 = shift out of output shift register to right. 0 = to left."]
pub type OUT_SHIFTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH_THRESH` reader - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
pub type PUSH_THRESH_R = crate::FieldReader;
#[doc = "Field `PUSH_THRESH` writer - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
pub type PUSH_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PULL_THRESH` reader - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
pub type PULL_THRESH_R = crate::FieldReader;
#[doc = "Field `PULL_THRESH` writer - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
pub type PULL_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FJOIN_TX` reader - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
pub type FJOIN_TX_R = crate::BitReader;
#[doc = "Field `FJOIN_TX` writer - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
pub type FJOIN_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FJOIN_RX` reader - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
pub type FJOIN_RX_R = crate::BitReader;
#[doc = "Field `FJOIN_RX` writer - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
pub type FJOIN_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
    #[inline(always)]
    pub fn in_count(&self) -> IN_COUNT_R {
        IN_COUNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 14 - If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub fn fjoin_rx_get(&self) -> FJOIN_RX_GET_R {
        FJOIN_RX_GET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub fn fjoin_rx_put(&self) -> FJOIN_RX_PUT_R {
        FJOIN_RX_PUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    pub fn autopush(&self) -> AUTOPUSH_R {
        AUTOPUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    pub fn autopull(&self) -> AUTOPULL_R {
        AUTOPULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn in_shiftdir(&self) -> IN_SHIFTDIR_R {
        IN_SHIFTDIR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn out_shiftdir(&self) -> OUT_SHIFTDIR_R {
        OUT_SHIFTDIR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub fn push_thresh(&self) -> PUSH_THRESH_R {
        PUSH_THRESH_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub fn pull_thresh(&self) -> PULL_THRESH_R {
        PULL_THRESH_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_tx(&self) -> FJOIN_TX_R {
        FJOIN_TX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_rx(&self) -> FJOIN_RX_R {
        FJOIN_RX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
    #[inline(always)]
    #[must_use]
    pub fn in_count(&mut self) -> IN_COUNT_W<SM_SHIFTCTRL_SPEC> {
        IN_COUNT_W::new(self, 0)
    }
    #[doc = "Bit 14 - If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    #[must_use]
    pub fn fjoin_rx_get(&mut self) -> FJOIN_RX_GET_W<SM_SHIFTCTRL_SPEC> {
        FJOIN_RX_GET_W::new(self, 14)
    }
    #[doc = "Bit 15 - If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    #[must_use]
    pub fn fjoin_rx_put(&mut self) -> FJOIN_RX_PUT_W<SM_SHIFTCTRL_SPEC> {
        FJOIN_RX_PUT_W::new(self, 15)
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    #[must_use]
    pub fn autopush(&mut self) -> AUTOPUSH_W<SM_SHIFTCTRL_SPEC> {
        AUTOPUSH_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    #[must_use]
    pub fn autopull(&mut self) -> AUTOPULL_W<SM_SHIFTCTRL_SPEC> {
        AUTOPULL_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    #[must_use]
    pub fn in_shiftdir(&mut self) -> IN_SHIFTDIR_W<SM_SHIFTCTRL_SPEC> {
        IN_SHIFTDIR_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    #[must_use]
    pub fn out_shiftdir(&mut self) -> OUT_SHIFTDIR_W<SM_SHIFTCTRL_SPEC> {
        OUT_SHIFTDIR_W::new(self, 19)
    }
    #[doc = "Bits 20:24 - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
    #[inline(always)]
    #[must_use]
    pub fn push_thresh(&mut self) -> PUSH_THRESH_W<SM_SHIFTCTRL_SPEC> {
        PUSH_THRESH_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
    #[inline(always)]
    #[must_use]
    pub fn pull_thresh(&mut self) -> PULL_THRESH_W<SM_SHIFTCTRL_SPEC> {
        PULL_THRESH_W::new(self, 25)
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    #[must_use]
    pub fn fjoin_tx(&mut self) -> FJOIN_TX_W<SM_SHIFTCTRL_SPEC> {
        FJOIN_TX_W::new(self, 30)
    }
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    #[must_use]
    pub fn fjoin_rx(&mut self) -> FJOIN_RX_W<SM_SHIFTCTRL_SPEC> {
        FJOIN_RX_W::new(self, 31)
    }
}
#[doc = "Control behaviour of the input/output shift registers for state machine 0  

You can [`read`](crate::Reg::read) this register and get [`sm_shiftctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sm_shiftctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_SHIFTCTRL_SPEC;
impl crate::RegisterSpec for SM_SHIFTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_shiftctrl::R`](R) reader structure"]
impl crate::Readable for SM_SHIFTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_shiftctrl::W`](W) writer structure"]
impl crate::Writable for SM_SHIFTCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SM_SHIFTCTRL to value 0x000c_0000"]
impl crate::Resettable for SM_SHIFTCTRL_SPEC {
    const RESET_VALUE: u32 = 0x000c_0000;
}
