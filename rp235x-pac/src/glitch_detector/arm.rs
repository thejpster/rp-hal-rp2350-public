#[doc = "Register `ARM` reader"]
pub type R = crate::R<ARM_SPEC>;
#[doc = "Register `ARM` writer"]
pub type W = crate::W<ARM_SPEC>;
#[doc = "  

Value on reset: 23469"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ARM_A {
    #[doc = "23469: Do not force the glitch detectors to be armed"]
    NO = 23469,
    #[doc = "0: Force the glitch detectors to be armed. (Any value other than ARM_NO counts as YES)"]
    YES = 0,
}
impl From<ARM_A> for u16 {
    #[inline(always)]
    fn from(variant: ARM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARM_A {
    type Ux = u16;
}
impl crate::IsEnum for ARM_A {}
#[doc = "Field `ARM` reader - "]
pub type ARM_R = crate::FieldReader<ARM_A>;
impl ARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARM_A> {
        match self.bits {
            23469 => Some(ARM_A::NO),
            0 => Some(ARM_A::YES),
            _ => None,
        }
    }
    #[doc = "Do not force the glitch detectors to be armed"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ARM_A::NO
    }
    #[doc = "Force the glitch detectors to be armed. (Any value other than ARM_NO counts as YES)"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ARM_A::YES
    }
}
#[doc = "Field `ARM` writer - "]
pub type ARM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, ARM_A>;
impl<'a, REG> ARM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Do not force the glitch detectors to be armed"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(ARM_A::NO)
    }
    #[doc = "Force the glitch detectors to be armed. (Any value other than ARM_NO counts as YES)"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(ARM_A::YES)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<ARM_SPEC> {
        ARM_W::new(self, 0)
    }
}
#[doc = "Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`arm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARM_SPEC;
impl crate::RegisterSpec for ARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arm::R`](R) reader structure"]
impl crate::Readable for ARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arm::W`](W) writer structure"]
impl crate::Writable for ARM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARM to value 0x5bad"]
impl crate::Resettable for ARM_SPEC {
    const RESET_VALUE: u32 = 0x5bad;
}
