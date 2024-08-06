#[doc = "Register `SW_LOCK55` reader"]
pub type R = crate::R<SW_LOCK55_SPEC>;
#[doc = "Register `SW_LOCK55` writer"]
pub type W = crate::W<SW_LOCK55_SPEC>;
#[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "0: `0`"]
    READ_WRITE = 0,
    #[doc = "1: `1`"]
    READ_ONLY = 1,
    #[doc = "3: `11`"]
    INACCESSIBLE = 3,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEC_A {
    type Ux = u8;
}
impl crate::IsEnum for SEC_A {}
#[doc = "Field `SEC` reader - Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
pub type SEC_R = crate::FieldReader<SEC_A>;
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEC_A> {
        match self.bits {
            0 => Some(SEC_A::READ_WRITE),
            1 => Some(SEC_A::READ_ONLY),
            3 => Some(SEC_A::INACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == SEC_A::READ_WRITE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == SEC_A::READ_ONLY
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == SEC_A::INACCESSIBLE
    }
}
#[doc = "Field `SEC` writer - Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
pub type SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SEC_A>;
impl<'a, REG> SEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(SEC_A::READ_WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(SEC_A::READ_ONLY)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn inaccessible(self) -> &'a mut crate::W<REG> {
        self.variant(SEC_A::INACCESSIBLE)
    }
}
#[doc = "Non-secure lock status. Writes are OR'd with the current value.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSEC_A {
    #[doc = "0: `0`"]
    READ_WRITE = 0,
    #[doc = "1: `1`"]
    READ_ONLY = 1,
    #[doc = "3: `11`"]
    INACCESSIBLE = 3,
}
impl From<NSEC_A> for u8 {
    #[inline(always)]
    fn from(variant: NSEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NSEC_A {
    type Ux = u8;
}
impl crate::IsEnum for NSEC_A {}
#[doc = "Field `NSEC` reader - Non-secure lock status. Writes are OR'd with the current value."]
pub type NSEC_R = crate::FieldReader<NSEC_A>;
impl NSEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NSEC_A> {
        match self.bits {
            0 => Some(NSEC_A::READ_WRITE),
            1 => Some(NSEC_A::READ_ONLY),
            3 => Some(NSEC_A::INACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == NSEC_A::READ_WRITE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == NSEC_A::READ_ONLY
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == NSEC_A::INACCESSIBLE
    }
}
#[doc = "Field `NSEC` writer - Non-secure lock status. Writes are OR'd with the current value."]
pub type NSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NSEC_A>;
impl<'a, REG> NSEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(NSEC_A::READ_WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(NSEC_A::READ_ONLY)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn inaccessible(self) -> &'a mut crate::W<REG> {
        self.variant(NSEC_A::INACCESSIBLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn nsec(&self) -> NSEC_R {
        NSEC_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<SW_LOCK55_SPEC> {
        SEC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    #[must_use]
    pub fn nsec(&mut self) -> NSEC_W<SW_LOCK55_SPEC> {
        NSEC_W::new(self, 2)
    }
}
#[doc = "Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_LOCK55_SPEC;
impl crate::RegisterSpec for SW_LOCK55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_lock55::R`](R) reader structure"]
impl crate::Readable for SW_LOCK55_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_lock55::W`](W) writer structure"]
impl crate::Writable for SW_LOCK55_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_LOCK55 to value 0"]
impl crate::Resettable for SW_LOCK55_SPEC {
    const RESET_VALUE: u32 = 0;
}
