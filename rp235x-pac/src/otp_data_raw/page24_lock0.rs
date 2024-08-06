#[doc = "Register `PAGE24_LOCK0` reader"]
pub type R = crate::R<PAGE24_LOCK0_SPEC>;
#[doc = "Register `PAGE24_LOCK0` writer"]
pub type W = crate::W<PAGE24_LOCK0_SPEC>;
#[doc = "Field `KEY_W` reader - Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
pub type KEY_W_R = crate::FieldReader;
#[doc = "Field `KEY_R` reader - Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
pub type KEY_R_R = crate::FieldReader;
#[doc = "State when at least one key is registered for this page and no matching key has been entered.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NO_KEY_STATE_A {
    #[doc = "0: `0`"]
    READ_ONLY = 0,
    #[doc = "1: `1`"]
    INACCESSIBLE = 1,
}
impl From<NO_KEY_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: NO_KEY_STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NO_KEY_STATE` reader - State when at least one key is registered for this page and no matching key has been entered."]
pub type NO_KEY_STATE_R = crate::BitReader<NO_KEY_STATE_A>;
impl NO_KEY_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NO_KEY_STATE_A {
        match self.bits {
            false => NO_KEY_STATE_A::READ_ONLY,
            true => NO_KEY_STATE_A::INACCESSIBLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == NO_KEY_STATE_A::READ_ONLY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == NO_KEY_STATE_A::INACCESSIBLE
    }
}
#[doc = "Field `R1` reader - Redundant copy of bits 7:0"]
pub type R1_R = crate::FieldReader;
#[doc = "Field `R2` reader - Redundant copy of bits 7:0"]
pub type R2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn key_w(&self) -> KEY_W_R {
        KEY_W_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn key_r(&self) -> KEY_R_R {
        KEY_R_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn no_key_state(&self) -> NO_KEY_STATE_R {
        NO_KEY_STATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page24_lock0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`page24_lock0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAGE24_LOCK0_SPEC;
impl crate::RegisterSpec for PAGE24_LOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`page24_lock0::R`](R) reader structure"]
impl crate::Readable for PAGE24_LOCK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`page24_lock0::W`](W) writer structure"]
impl crate::Writable for PAGE24_LOCK0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAGE24_LOCK0 to value 0"]
impl crate::Resettable for PAGE24_LOCK0_SPEC {
    const RESET_VALUE: u32 = 0;
}
