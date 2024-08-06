#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Register `STATE` writer"]
pub type W = crate::W<STATE_SPEC>;
#[doc = "Field `CURRENT` reader - "]
pub type CURRENT_R = crate::FieldReader;
#[doc = "Field `REQ` reader - "]
pub type REQ_R = crate::FieldReader;
#[doc = "Field `REQ` writer - "]
pub type REQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REQ_IGNORED` reader - "]
pub type REQ_IGNORED_R = crate::BitReader;
#[doc = "Field `REQ_IGNORED` writer - "]
pub type REQ_IGNORED_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PWRUP_WHILE_WAITING` reader - Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
pub type PWRUP_WHILE_WAITING_R = crate::BitReader;
#[doc = "Field `PWRUP_WHILE_WAITING` writer - Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
pub type PWRUP_WHILE_WAITING_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BAD_SW_REQ` reader - Bad software initiated state request. No action taken."]
pub type BAD_SW_REQ_R = crate::BitReader;
#[doc = "Field `BAD_HW_REQ` reader - Bad hardware initiated state request. Went back to state 0 (i.e. everything powered up)"]
pub type BAD_HW_REQ_R = crate::BitReader;
#[doc = "Field `WAITING` reader - "]
pub type WAITING_R = crate::BitReader;
#[doc = "Field `CHANGING` reader - "]
pub type CHANGING_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn req_ignored(&self) -> REQ_IGNORED_R {
        REQ_IGNORED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> PWRUP_WHILE_WAITING_R {
        PWRUP_WHILE_WAITING_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bad software initiated state request. No action taken."]
    #[inline(always)]
    pub fn bad_sw_req(&self) -> BAD_SW_REQ_R {
        BAD_SW_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bad hardware initiated state request. Went back to state 0 (i.e. everything powered up)"]
    #[inline(always)]
    pub fn bad_hw_req(&self) -> BAD_HW_REQ_R {
        BAD_HW_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn changing(&self) -> CHANGING_R {
        CHANGING_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<STATE_SPEC> {
        REQ_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn req_ignored(&mut self) -> REQ_IGNORED_W<STATE_SPEC> {
        REQ_IGNORED_W::new(self, 8)
    }
    #[doc = "Bit 9 - Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_while_waiting(&mut self) -> PWRUP_WHILE_WAITING_W<STATE_SPEC> {
        PWRUP_WHILE_WAITING_W::new(self, 9)
    }
}
#[doc = "This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT &amp; POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes.  

You can [`read`](crate::Reg::read) this register and get [`state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`state::W`](W) writer structure"]
impl crate::Writable for STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0300;
}
#[doc = "`reset()` method sets STATE to value 0x0f"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
