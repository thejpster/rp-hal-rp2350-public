#[doc = "Register `SENSITIVITY` reader"]
pub type R = crate::R<SENSITIVITY_SPEC>;
#[doc = "Register `SENSITIVITY` writer"]
pub type W = crate::W<SENSITIVITY_SPEC>;
#[doc = "Field `DET0` reader - Set sensitivity for detector 0. Higher values are more sensitive."]
pub type DET0_R = crate::FieldReader;
#[doc = "Field `DET0` writer - Set sensitivity for detector 0. Higher values are more sensitive."]
pub type DET0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET1` reader - Set sensitivity for detector 1. Higher values are more sensitive."]
pub type DET1_R = crate::FieldReader;
#[doc = "Field `DET1` writer - Set sensitivity for detector 1. Higher values are more sensitive."]
pub type DET1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET2` reader - Set sensitivity for detector 2. Higher values are more sensitive."]
pub type DET2_R = crate::FieldReader;
#[doc = "Field `DET2` writer - Set sensitivity for detector 2. Higher values are more sensitive."]
pub type DET2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET3` reader - Set sensitivity for detector 3. Higher values are more sensitive."]
pub type DET3_R = crate::FieldReader;
#[doc = "Field `DET3` writer - Set sensitivity for detector 3. Higher values are more sensitive."]
pub type DET3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET0_INV` reader - Must be the inverse of DET0, else the default value is used."]
pub type DET0_INV_R = crate::FieldReader;
#[doc = "Field `DET0_INV` writer - Must be the inverse of DET0, else the default value is used."]
pub type DET0_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET1_INV` reader - Must be the inverse of DET1, else the default value is used."]
pub type DET1_INV_R = crate::FieldReader;
#[doc = "Field `DET1_INV` writer - Must be the inverse of DET1, else the default value is used."]
pub type DET1_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET2_INV` reader - Must be the inverse of DET2, else the default value is used."]
pub type DET2_INV_R = crate::FieldReader;
#[doc = "Field `DET2_INV` writer - Must be the inverse of DET2, else the default value is used."]
pub type DET2_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DET3_INV` reader - Must be the inverse of DET3, else the default value is used."]
pub type DET3_INV_R = crate::FieldReader;
#[doc = "Field `DET3_INV` writer - Must be the inverse of DET3, else the default value is used."]
pub type DET3_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFAULT_A {
    #[doc = "0: Use the default sensitivity configured in OTP for all detectors. (Any value other than DEFAULT_NO counts as YES)"]
    YES = 0,
    #[doc = "222: Do not use the default sensitivity configured in OTP. Instead use the value from this register."]
    NO = 222,
}
impl From<DEFAULT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFAULT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEFAULT_A {
    type Ux = u8;
}
impl crate::IsEnum for DEFAULT_A {}
#[doc = "Field `DEFAULT` reader - "]
pub type DEFAULT_R = crate::FieldReader<DEFAULT_A>;
impl DEFAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DEFAULT_A> {
        match self.bits {
            0 => Some(DEFAULT_A::YES),
            222 => Some(DEFAULT_A::NO),
            _ => None,
        }
    }
    #[doc = "Use the default sensitivity configured in OTP for all detectors. (Any value other than DEFAULT_NO counts as YES)"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DEFAULT_A::YES
    }
    #[doc = "Do not use the default sensitivity configured in OTP. Instead use the value from this register."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DEFAULT_A::NO
    }
}
#[doc = "Field `DEFAULT` writer - "]
pub type DEFAULT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DEFAULT_A>;
impl<'a, REG> DEFAULT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use the default sensitivity configured in OTP for all detectors. (Any value other than DEFAULT_NO counts as YES)"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DEFAULT_A::YES)
    }
    #[doc = "Do not use the default sensitivity configured in OTP. Instead use the value from this register."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DEFAULT_A::NO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set sensitivity for detector 0. Higher values are more sensitive."]
    #[inline(always)]
    pub fn det0(&self) -> DET0_R {
        DET0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set sensitivity for detector 1. Higher values are more sensitive."]
    #[inline(always)]
    pub fn det1(&self) -> DET1_R {
        DET1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set sensitivity for detector 2. Higher values are more sensitive."]
    #[inline(always)]
    pub fn det2(&self) -> DET2_R {
        DET2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Set sensitivity for detector 3. Higher values are more sensitive."]
    #[inline(always)]
    pub fn det3(&self) -> DET3_R {
        DET3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Must be the inverse of DET0, else the default value is used."]
    #[inline(always)]
    pub fn det0_inv(&self) -> DET0_INV_R {
        DET0_INV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Must be the inverse of DET1, else the default value is used."]
    #[inline(always)]
    pub fn det1_inv(&self) -> DET1_INV_R {
        DET1_INV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Must be the inverse of DET2, else the default value is used."]
    #[inline(always)]
    pub fn det2_inv(&self) -> DET2_INV_R {
        DET2_INV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Must be the inverse of DET3, else the default value is used."]
    #[inline(always)]
    pub fn det3_inv(&self) -> DET3_INV_R {
        DET3_INV_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn default(&self) -> DEFAULT_R {
        DEFAULT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set sensitivity for detector 0. Higher values are more sensitive."]
    #[inline(always)]
    #[must_use]
    pub fn det0(&mut self) -> DET0_W<SENSITIVITY_SPEC> {
        DET0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set sensitivity for detector 1. Higher values are more sensitive."]
    #[inline(always)]
    #[must_use]
    pub fn det1(&mut self) -> DET1_W<SENSITIVITY_SPEC> {
        DET1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Set sensitivity for detector 2. Higher values are more sensitive."]
    #[inline(always)]
    #[must_use]
    pub fn det2(&mut self) -> DET2_W<SENSITIVITY_SPEC> {
        DET2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Set sensitivity for detector 3. Higher values are more sensitive."]
    #[inline(always)]
    #[must_use]
    pub fn det3(&mut self) -> DET3_W<SENSITIVITY_SPEC> {
        DET3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Must be the inverse of DET0, else the default value is used."]
    #[inline(always)]
    #[must_use]
    pub fn det0_inv(&mut self) -> DET0_INV_W<SENSITIVITY_SPEC> {
        DET0_INV_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Must be the inverse of DET1, else the default value is used."]
    #[inline(always)]
    #[must_use]
    pub fn det1_inv(&mut self) -> DET1_INV_W<SENSITIVITY_SPEC> {
        DET1_INV_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Must be the inverse of DET2, else the default value is used."]
    #[inline(always)]
    #[must_use]
    pub fn det2_inv(&mut self) -> DET2_INV_W<SENSITIVITY_SPEC> {
        DET2_INV_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Must be the inverse of DET3, else the default value is used."]
    #[inline(always)]
    #[must_use]
    pub fn det3_inv(&mut self) -> DET3_INV_W<SENSITIVITY_SPEC> {
        DET3_INV_W::new(self, 14)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn default(&mut self) -> DEFAULT_W<SENSITIVITY_SPEC> {
        DEFAULT_W::new(self, 24)
    }
}
#[doc = "Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`sensitivity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensitivity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENSITIVITY_SPEC;
impl crate::RegisterSpec for SENSITIVITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensitivity::R`](R) reader structure"]
impl crate::Readable for SENSITIVITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sensitivity::W`](W) writer structure"]
impl crate::Writable for SENSITIVITY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SENSITIVITY to value 0"]
impl crate::Resettable for SENSITIVITY_SPEC {
    const RESET_VALUE: u32 = 0;
}
