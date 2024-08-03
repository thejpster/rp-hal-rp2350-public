#[doc = "Register `MTVEC` reader"]
pub type R = crate::R<MTVEC_SPEC>;
#[doc = "Register `MTVEC` writer"]
pub type W = crate::W<MTVEC_SPEC>;
#[doc = "If 0 (direct mode), all traps set pc to the trap vector base. If 1 (vectored), exceptions set pc to the trap vector base, and interrupts set pc to 4 times the interrupt cause (3=soft IRQ, 7=timer IRQ, 11=external IRQ).  

 The upper bit is hardwired to zero, so attempting to set mode to 2 or 3 will result in a value of 0 or 1 respectively.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Direct entry to mtvec"]
    DIRECT = 0,
    #[doc = "1: Vectored entry to a 16-entry jump table starting at mtvec"]
    VECTORED = 1,
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
#[doc = "Field `MODE` reader - If 0 (direct mode), all traps set pc to the trap vector base. If 1 (vectored), exceptions set pc to the trap vector base, and interrupts set pc to 4 times the interrupt cause (3=soft IRQ, 7=timer IRQ, 11=external IRQ).  

 The upper bit is hardwired to zero, so attempting to set mode to 2 or 3 will result in a value of 0 or 1 respectively."]
pub type MODE_R = crate::FieldReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DIRECT),
            1 => Some(MODE_A::VECTORED),
            _ => None,
        }
    }
    #[doc = "Direct entry to mtvec"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == MODE_A::DIRECT
    }
    #[doc = "Vectored entry to a 16-entry jump table starting at mtvec"]
    #[inline(always)]
    pub fn is_vectored(&self) -> bool {
        *self == MODE_A::VECTORED
    }
}
#[doc = "Field `MODE` writer - If 0 (direct mode), all traps set pc to the trap vector base. If 1 (vectored), exceptions set pc to the trap vector base, and interrupts set pc to 4 times the interrupt cause (3=soft IRQ, 7=timer IRQ, 11=external IRQ).  

 The upper bit is hardwired to zero, so attempting to set mode to 2 or 3 will result in a value of 0 or 1 respectively."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct entry to mtvec"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DIRECT)
    }
    #[doc = "Vectored entry to a 16-entry jump table starting at mtvec"]
    #[inline(always)]
    pub fn vectored(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::VECTORED)
    }
}
#[doc = "Field `BASE` reader - The upper 30 bits of the trap vector address (2 LSBs are implicitly 0). Must be 64-byte-aligned if vectoring is enabled. Otherwise, must be 4-byte-aligned."]
pub type BASE_R = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - The upper 30 bits of the trap vector address (2 LSBs are implicitly 0). Must be 64-byte-aligned if vectoring is enabled. Otherwise, must be 4-byte-aligned."]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - If 0 (direct mode), all traps set pc to the trap vector base. If 1 (vectored), exceptions set pc to the trap vector base, and interrupts set pc to 4 times the interrupt cause (3=soft IRQ, 7=timer IRQ, 11=external IRQ).  

 The upper bit is hardwired to zero, so attempting to set mode to 2 or 3 will result in a value of 0 or 1 respectively."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - The upper 30 bits of the trap vector address (2 LSBs are implicitly 0). Must be 64-byte-aligned if vectoring is enabled. Otherwise, must be 4-byte-aligned."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - If 0 (direct mode), all traps set pc to the trap vector base. If 1 (vectored), exceptions set pc to the trap vector base, and interrupts set pc to 4 times the interrupt cause (3=soft IRQ, 7=timer IRQ, 11=external IRQ).  

 The upper bit is hardwired to zero, so attempting to set mode to 2 or 3 will result in a value of 0 or 1 respectively."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<MTVEC_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:31 - The upper 30 bits of the trap vector address (2 LSBs are implicitly 0). Must be 64-byte-aligned if vectoring is enabled. Otherwise, must be 4-byte-aligned."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<MTVEC_SPEC> {
        BASE_W::new(self, 2)
    }
}
#[doc = "Machine trap handler base address.  

You can [`read`](crate::Reg::read) this register and get [`mtvec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtvec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTVEC_SPEC;
impl crate::RegisterSpec for MTVEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtvec::R`](R) reader structure"]
impl crate::Readable for MTVEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtvec::W`](W) writer structure"]
impl crate::Writable for MTVEC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTVEC to value 0x7ffc"]
impl crate::Resettable for MTVEC_SPEC {
    const RESET_VALUE: u32 = 0x7ffc;
}
