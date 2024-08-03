#[doc = "Register `DBG_POW_STATE_XIP` reader"]
pub type R = crate::R<DBG_POW_STATE_XIP_SPEC>;
#[doc = "Field `IS_PD` reader - Indicates the power somain is fully powered down."]
pub type IS_PD_R = crate::BitReader;
#[doc = "Field `SMALL_ACK0` reader - This bit indicates the state of the last element in small power switch chain 0."]
pub type SMALL_ACK0_R = crate::BitReader;
#[doc = "Field `SMALL_ACK1` reader - This bit indicates the state of the last element in small power switch chain 1."]
pub type SMALL_ACK1_R = crate::BitReader;
#[doc = "Field `SMALL_ACK2` reader - The small switches are split into 3 chains. In the power up sequence they are switched on separately to allow management of the VDD rise time. In the power down sequence they switch off simultaneously with the large power switches.  
 This bit indicates the state of the last element in small power switch chain 2."]
pub type SMALL_ACK2_R = crate::BitReader;
#[doc = "Field `LARGE_ACK` reader - Indicates the state of the large power switches for the power domain."]
pub type LARGE_ACK_R = crate::BitReader;
#[doc = "Field `ISOLATE_FROM_SEQ` reader - Indicates the state of the isolation control to the power domain."]
pub type ISOLATE_FROM_SEQ_R = crate::BitReader;
#[doc = "Field `ENAB_ACK` reader - Indicates the state of the enable to the power domain."]
pub type ENAB_ACK_R = crate::BitReader;
#[doc = "Field `RESET_FROM_SEQ` reader - Indicates the state of the reset to the power domain."]
pub type RESET_FROM_SEQ_R = crate::BitReader;
#[doc = "Field `IS_PU` reader - Indicates the power somain is fully powered up."]
pub type IS_PU_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the power somain is fully powered down."]
    #[inline(always)]
    pub fn is_pd(&self) -> IS_PD_R {
        IS_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the state of the last element in small power switch chain 0."]
    #[inline(always)]
    pub fn small_ack0(&self) -> SMALL_ACK0_R {
        SMALL_ACK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates the state of the last element in small power switch chain 1."]
    #[inline(always)]
    pub fn small_ack1(&self) -> SMALL_ACK1_R {
        SMALL_ACK1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The small switches are split into 3 chains. In the power up sequence they are switched on separately to allow management of the VDD rise time. In the power down sequence they switch off simultaneously with the large power switches.  
 This bit indicates the state of the last element in small power switch chain 2."]
    #[inline(always)]
    pub fn small_ack2(&self) -> SMALL_ACK2_R {
        SMALL_ACK2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the state of the large power switches for the power domain."]
    #[inline(always)]
    pub fn large_ack(&self) -> LARGE_ACK_R {
        LARGE_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the state of the isolation control to the power domain."]
    #[inline(always)]
    pub fn isolate_from_seq(&self) -> ISOLATE_FROM_SEQ_R {
        ISOLATE_FROM_SEQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the state of the enable to the power domain."]
    #[inline(always)]
    pub fn enab_ack(&self) -> ENAB_ACK_R {
        ENAB_ACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the state of the reset to the power domain."]
    #[inline(always)]
    pub fn reset_from_seq(&self) -> RESET_FROM_SEQ_R {
        RESET_FROM_SEQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the power somain is fully powered up."]
    #[inline(always)]
    pub fn is_pu(&self) -> IS_PU_R {
        IS_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "This register indicates the state of the power sequencer for the XIP domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_state_xip::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_POW_STATE_XIP_SPEC;
impl crate::RegisterSpec for DBG_POW_STATE_XIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_pow_state_xip::R`](R) reader structure"]
impl crate::Readable for DBG_POW_STATE_XIP_SPEC {}
#[doc = "`reset()` method sets DBG_POW_STATE_XIP to value 0"]
impl crate::Resettable for DBG_POW_STATE_XIP_SPEC {
    const RESET_VALUE: u32 = 0;
}
