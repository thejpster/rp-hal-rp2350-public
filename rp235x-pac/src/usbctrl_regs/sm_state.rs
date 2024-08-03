#[doc = "Register `SM_STATE` reader"]
pub type R = crate::R<SM_STATE_SPEC>;
#[doc = "Field `STATE` reader - "]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `BC_STATE` reader - "]
pub type BC_STATE_R = crate::FieldReader;
#[doc = "Field `RX_DASM` reader - "]
pub type RX_DASM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn bc_state(&self) -> BC_STATE_R {
        BC_STATE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rx_dasm(&self) -> RX_DASM_R {
        RX_DASM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`sm_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_STATE_SPEC;
impl crate::RegisterSpec for SM_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_state::R`](R) reader structure"]
impl crate::Readable for SM_STATE_SPEC {}
#[doc = "`reset()` method sets SM_STATE to value 0"]
impl crate::Resettable for SM_STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
