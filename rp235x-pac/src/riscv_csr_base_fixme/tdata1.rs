#[doc = "Register `TDATA1` reader"]
pub type R = crate::R<TDATA1_SPEC>;
#[doc = "Register `TDATA1` writer"]
pub type W = crate::W<TDATA1_SPEC>;
#[doc = "Field `LOAD` reader - Hardwired to 0 to indicate load address/data triggers are not supported"]
pub type LOAD_R = crate::BitReader;
#[doc = "Field `STORE` reader - Hardwired to 0 to indicate store address/data triggers are not supported"]
pub type STORE_R = crate::BitReader;
#[doc = "Field `EXECUTE` reader - When set, the trigger fires on the address of an instruction that is executed."]
pub type EXECUTE_R = crate::BitReader;
#[doc = "Field `EXECUTE` writer - When set, the trigger fires on the address of an instruction that is executed."]
pub type EXECUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U` reader - When set, enable this trigger in U-mode"]
pub type U_R = crate::BitReader;
#[doc = "Field `U` writer - When set, enable this trigger in U-mode"]
pub type U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - When set, enable this trigger in M-mode"]
pub type M_R = crate::BitReader;
#[doc = "Field `M` writer - When set, enable this trigger in M-mode"]
pub type M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH` reader - Hardwired to 0 to indicate match is always on the full address specified by `tdata2`"]
pub type MATCH_R = crate::FieldReader;
#[doc = "Field `CHAIN` reader - Hardwired to 0 to indicate trigger chaining is not supported."]
pub type CHAIN_R = crate::BitReader;
#[doc = "Select action to be taken when the trigger fires.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTION_A {
    #[doc = "0: Raise a breakpoint exception, which can be handled by the M-mode exception handler"]
    EBREAK = 0,
    #[doc = "1: Enter debug mode. This action is only selectable when `tdata1.dmode` is 1."]
    DEBUG = 1,
}
impl From<ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTION_A {
    type Ux = u8;
}
impl crate::IsEnum for ACTION_A {}
#[doc = "Field `ACTION` reader - Select action to be taken when the trigger fires."]
pub type ACTION_R = crate::FieldReader<ACTION_A>;
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACTION_A> {
        match self.bits {
            0 => Some(ACTION_A::EBREAK),
            1 => Some(ACTION_A::DEBUG),
            _ => None,
        }
    }
    #[doc = "Raise a breakpoint exception, which can be handled by the M-mode exception handler"]
    #[inline(always)]
    pub fn is_ebreak(&self) -> bool {
        *self == ACTION_A::EBREAK
    }
    #[doc = "Enter debug mode. This action is only selectable when `tdata1.dmode` is 1."]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == ACTION_A::DEBUG
    }
}
#[doc = "Field `ACTION` writer - Select action to be taken when the trigger fires."]
pub type ACTION_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ACTION_A>;
impl<'a, REG> ACTION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Raise a breakpoint exception, which can be handled by the M-mode exception handler"]
    #[inline(always)]
    pub fn ebreak(self) -> &'a mut crate::W<REG> {
        self.variant(ACTION_A::EBREAK)
    }
    #[doc = "Enter debug mode. This action is only selectable when `tdata1.dmode` is 1."]
    #[inline(always)]
    pub fn debug(self) -> &'a mut crate::W<REG> {
        self.variant(ACTION_A::DEBUG)
    }
}
#[doc = "Field `SIZELO` reader - Hardwired value of 0 indicates that access size matching is not supported"]
pub type SIZELO_R = crate::FieldReader;
#[doc = "Field `TIMING` reader - Hardwired value of 0 indicates that trigger fires before the triggering instruction executes, not afterward"]
pub type TIMING_R = crate::BitReader;
#[doc = "Field `SELECT` reader - Hardwired value of 0 indicates that only address matches are supported, not data matches"]
pub type SELECT_R = crate::BitReader;
#[doc = "Field `HIT` reader - Trigger hit flag. Not implemented, hardwired to 0."]
pub type HIT_R = crate::BitReader;
#[doc = "Field `MASKMAX` reader - Value of 0 indicates only exact address matches are supported"]
pub type MASKMAX_R = crate::FieldReader;
#[doc = "Field `DMODE` reader - If 0, both Debug and M-mode can write the `tdata` registers at the selected `tselect`.  

 If 1, only Debug Mode can write the `tdata` registers at the selected `tselect`. Writes from other modes are ignored.  

 This bit is only writable from Debug Mode"]
pub type DMODE_R = crate::BitReader;
#[doc = "Field `DMODE` writer - If 0, both Debug and M-mode can write the `tdata` registers at the selected `tselect`.  

 If 1, only Debug Mode can write the `tdata` registers at the selected `tselect`. Writes from other modes are ignored.  

 This bit is only writable from Debug Mode"]
pub type DMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE` reader - Trigger type. Hardwired to type=2, meaning an address/data match trigger"]
pub type TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Hardwired to 0 to indicate load address/data triggers are not supported"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardwired to 0 to indicate store address/data triggers are not supported"]
    #[inline(always)]
    pub fn store(&self) -> STORE_R {
        STORE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, the trigger fires on the address of an instruction that is executed."]
    #[inline(always)]
    pub fn execute(&self) -> EXECUTE_R {
        EXECUTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, enable this trigger in U-mode"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - When set, enable this trigger in M-mode"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Hardwired to 0 to indicate match is always on the full address specified by `tdata2`"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Hardwired to 0 to indicate trigger chaining is not supported."]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Select action to be taken when the trigger fires."]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Hardwired value of 0 indicates that access size matching is not supported"]
    #[inline(always)]
    pub fn sizelo(&self) -> SIZELO_R {
        SIZELO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Hardwired value of 0 indicates that trigger fires before the triggering instruction executes, not afterward"]
    #[inline(always)]
    pub fn timing(&self) -> TIMING_R {
        TIMING_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hardwired value of 0 indicates that only address matches are supported, not data matches"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Trigger hit flag. Not implemented, hardwired to 0."]
    #[inline(always)]
    pub fn hit(&self) -> HIT_R {
        HIT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:26 - Value of 0 indicates only exact address matches are supported"]
    #[inline(always)]
    pub fn maskmax(&self) -> MASKMAX_R {
        MASKMAX_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bit 27 - If 0, both Debug and M-mode can write the `tdata` registers at the selected `tselect`.  

 If 1, only Debug Mode can write the `tdata` registers at the selected `tselect`. Writes from other modes are ignored.  

 This bit is only writable from Debug Mode"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Trigger type. Hardwired to type=2, meaning an address/data match trigger"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - When set, the trigger fires on the address of an instruction that is executed."]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> EXECUTE_W<TDATA1_SPEC> {
        EXECUTE_W::new(self, 2)
    }
    #[doc = "Bit 3 - When set, enable this trigger in U-mode"]
    #[inline(always)]
    #[must_use]
    pub fn u(&mut self) -> U_W<TDATA1_SPEC> {
        U_W::new(self, 3)
    }
    #[doc = "Bit 6 - When set, enable this trigger in M-mode"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<TDATA1_SPEC> {
        M_W::new(self, 6)
    }
    #[doc = "Bits 12:15 - Select action to be taken when the trigger fires."]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<TDATA1_SPEC> {
        ACTION_W::new(self, 12)
    }
    #[doc = "Bit 27 - If 0, both Debug and M-mode can write the `tdata` registers at the selected `tselect`.  

 If 1, only Debug Mode can write the `tdata` registers at the selected `tselect`. Writes from other modes are ignored.  

 This bit is only writable from Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<TDATA1_SPEC> {
        DMODE_W::new(self, 27)
    }
}
#[doc = "Trigger configuration data 1  

 Hazard 3 only supports address/data match triggers (type=2) so this register description includes the `mcontrol` fields for this type.  

 More precisely, Hazard3 only supports exact instruction address match triggers (hardware breakpoints) so many of this register's fields are hardwired.  

You can [`read`](crate::Reg::read) this register and get [`tdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDATA1_SPEC;
impl crate::RegisterSpec for TDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdata1::R`](R) reader structure"]
impl crate::Readable for TDATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdata1::W`](W) writer structure"]
impl crate::Writable for TDATA1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDATA1 to value 0x2000_0000"]
impl crate::Resettable for TDATA1_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
