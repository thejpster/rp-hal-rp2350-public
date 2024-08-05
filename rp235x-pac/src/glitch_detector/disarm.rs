#[doc = "Register `DISARM` reader"]
pub type R = crate::R<DISARM_SPEC>;
#[doc = "Register `DISARM` writer"]
pub type W = crate::W<DISARM_SPEC>;
#[doc = "Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DISARM_A {
    #[doc = "0: Do not disarm the glitch detectors. (Any value other than DISARM_YES counts as NO)"]
    NO = 0,
    #[doc = "56495: Disarm the glitch detectors"]
    YES = 56495,
}
impl From<DISARM_A> for u16 {
    #[inline(always)]
    fn from(variant: DISARM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DISARM_A {
    type Ux = u16;
}
impl crate::IsEnum for DISARM_A {}
#[doc = "Field `DISARM` reader - Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
pub type DISARM_R = crate::FieldReader<DISARM_A>;
impl DISARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DISARM_A> {
        match self.bits {
            0 => Some(DISARM_A::NO),
            56495 => Some(DISARM_A::YES),
            _ => None,
        }
    }
    #[doc = "Do not disarm the glitch detectors. (Any value other than DISARM_YES counts as NO)"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DISARM_A::NO
    }
    #[doc = "Disarm the glitch detectors"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DISARM_A::YES
    }
}
#[doc = "Field `DISARM` writer - Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
pub type DISARM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, DISARM_A>;
impl<'a, REG> DISARM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Do not disarm the glitch detectors. (Any value other than DISARM_YES counts as NO)"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DISARM_A::NO)
    }
    #[doc = "Disarm the glitch detectors"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DISARM_A::YES)
    }
}
impl R {
    #[doc = "Bits 0:15 - Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
    #[inline(always)]
    pub fn disarm(&self) -> DISARM_R {
        DISARM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
    #[inline(always)]
    #[must_use]
    pub fn disarm(&mut self) -> DISARM_W<DISARM_SPEC> {
        DISARM_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`disarm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disarm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DISARM_SPEC;
impl crate::RegisterSpec for DISARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disarm::R`](R) reader structure"]
impl crate::Readable for DISARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`disarm::W`](W) writer structure"]
impl crate::Writable for DISARM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISARM to value 0"]
impl crate::Resettable for DISARM_SPEC {
    const RESET_VALUE: u32 = 0;
}
