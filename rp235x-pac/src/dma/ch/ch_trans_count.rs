#[doc = "Register `CH_TRANS_COUNT` reader"]
pub type R = crate::R<CH_TRANS_COUNT_SPEC>;
#[doc = "Register `CH_TRANS_COUNT` writer"]
pub type W = crate::W<CH_TRANS_COUNT_SPEC>;
#[doc = "Field `COUNT` reader - 28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    TRIGGER_SELF = 1,
    #[doc = "15: `1111`"]
    ENDLESS = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE_A {}
#[doc = "Field `MODE` reader - When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
pub type MODE_R = crate::FieldReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::NORMAL),
            1 => Some(MODE_A::TRIGGER_SELF),
            15 => Some(MODE_A::ENDLESS),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_trigger_self(&self) -> bool {
        *self == MODE_A::TRIGGER_SELF
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_endless(&self) -> bool {
        *self == MODE_A::ENDLESS
    }
}
#[doc = "Field `MODE` writer - When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn trigger_self(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::TRIGGER_SELF)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn endless(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ENDLESS)
    }
}
impl R {
    #[doc = "Bits 0:27 - 28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - 28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<CH_TRANS_COUNT_SPEC> {
        COUNT_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CH_TRANS_COUNT_SPEC> {
        MODE_W::new(self, 28)
    }
}
#[doc = "DMA Channel 0 Transfer Count  

You can [`read`](crate::Reg::read) this register and get [`ch_trans_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_trans_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_trans_count::R`](R) reader structure"]
impl crate::Readable for CH_TRANS_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_trans_count::W`](W) writer structure"]
impl crate::Writable for CH_TRANS_COUNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_TRANS_COUNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
