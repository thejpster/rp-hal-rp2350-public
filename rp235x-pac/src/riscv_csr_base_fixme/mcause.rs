#[doc = "Register `MCAUSE` reader"]
pub type R = crate::R<MCAUSE_SPEC>;
#[doc = "Register `MCAUSE` writer"]
pub type W = crate::W<MCAUSE_SPEC>;
#[doc = "If `interrupt` is set, `code` indicates the index of the bit in mip that caused the trap (3=soft IRQ, 7=timer IRQ, 11=external IRQ). Otherwise, `code` is set according to the cause of the exception.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CODE_A {
    #[doc = "0: Instruction fetch was misaligned. Will never fire on RP2350, since the C extension is enabled."]
    INSTR_ALIGN = 0,
    #[doc = "1: Instruction access fault. Instruction fetch failed a PMP check, or encountered a downstream bus fault, and then passed the point of no speculation."]
    INSTR_FAULT = 1,
    #[doc = "2: Illegal instruction was executed (including illegal CSR accesses)"]
    ILLEGAL_INSTR = 2,
    #[doc = "3: Breakpoint. An ebreak instruction was executed when the relevant dcsr.ebreak bit was clear."]
    BREAKPOINT = 3,
    #[doc = "4: Load address misaligned. Hazard3 requires natural alignment of all accesses."]
    LOAD_ALIGN = 4,
    #[doc = "5: Load access fault. A load failed a PMP check, or encountered a downstream bus error."]
    LOAD_FAULT = 5,
    #[doc = "6: Store/AMO address misaligned. Hazard3 requires natural alignment of all accesses."]
    STORE_ALIGN = 6,
    #[doc = "7: Store/AMO access fault. A store/AMO failed a PMP check, or encountered a downstream bus error. Also set if an AMO is attempted on a region that does not support atomics (on RP2350, anything but SRAM)."]
    STORE_FAULT = 7,
    #[doc = "8: Environment call from U-mode."]
    U_ECALL = 8,
    #[doc = "11: Environment call from M-mode."]
    M_ECALL = 11,
}
impl From<CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CODE_A {
    type Ux = u8;
}
impl crate::IsEnum for CODE_A {}
#[doc = "Field `CODE` reader - If `interrupt` is set, `code` indicates the index of the bit in mip that caused the trap (3=soft IRQ, 7=timer IRQ, 11=external IRQ). Otherwise, `code` is set according to the cause of the exception."]
pub type CODE_R = crate::FieldReader<CODE_A>;
impl CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CODE_A> {
        match self.bits {
            0 => Some(CODE_A::INSTR_ALIGN),
            1 => Some(CODE_A::INSTR_FAULT),
            2 => Some(CODE_A::ILLEGAL_INSTR),
            3 => Some(CODE_A::BREAKPOINT),
            4 => Some(CODE_A::LOAD_ALIGN),
            5 => Some(CODE_A::LOAD_FAULT),
            6 => Some(CODE_A::STORE_ALIGN),
            7 => Some(CODE_A::STORE_FAULT),
            8 => Some(CODE_A::U_ECALL),
            11 => Some(CODE_A::M_ECALL),
            _ => None,
        }
    }
    #[doc = "Instruction fetch was misaligned. Will never fire on RP2350, since the C extension is enabled."]
    #[inline(always)]
    pub fn is_instr_align(&self) -> bool {
        *self == CODE_A::INSTR_ALIGN
    }
    #[doc = "Instruction access fault. Instruction fetch failed a PMP check, or encountered a downstream bus fault, and then passed the point of no speculation."]
    #[inline(always)]
    pub fn is_instr_fault(&self) -> bool {
        *self == CODE_A::INSTR_FAULT
    }
    #[doc = "Illegal instruction was executed (including illegal CSR accesses)"]
    #[inline(always)]
    pub fn is_illegal_instr(&self) -> bool {
        *self == CODE_A::ILLEGAL_INSTR
    }
    #[doc = "Breakpoint. An ebreak instruction was executed when the relevant dcsr.ebreak bit was clear."]
    #[inline(always)]
    pub fn is_breakpoint(&self) -> bool {
        *self == CODE_A::BREAKPOINT
    }
    #[doc = "Load address misaligned. Hazard3 requires natural alignment of all accesses."]
    #[inline(always)]
    pub fn is_load_align(&self) -> bool {
        *self == CODE_A::LOAD_ALIGN
    }
    #[doc = "Load access fault. A load failed a PMP check, or encountered a downstream bus error."]
    #[inline(always)]
    pub fn is_load_fault(&self) -> bool {
        *self == CODE_A::LOAD_FAULT
    }
    #[doc = "Store/AMO address misaligned. Hazard3 requires natural alignment of all accesses."]
    #[inline(always)]
    pub fn is_store_align(&self) -> bool {
        *self == CODE_A::STORE_ALIGN
    }
    #[doc = "Store/AMO access fault. A store/AMO failed a PMP check, or encountered a downstream bus error. Also set if an AMO is attempted on a region that does not support atomics (on RP2350, anything but SRAM)."]
    #[inline(always)]
    pub fn is_store_fault(&self) -> bool {
        *self == CODE_A::STORE_FAULT
    }
    #[doc = "Environment call from U-mode."]
    #[inline(always)]
    pub fn is_u_ecall(&self) -> bool {
        *self == CODE_A::U_ECALL
    }
    #[doc = "Environment call from M-mode."]
    #[inline(always)]
    pub fn is_m_ecall(&self) -> bool {
        *self == CODE_A::M_ECALL
    }
}
#[doc = "Field `CODE` writer - If `interrupt` is set, `code` indicates the index of the bit in mip that caused the trap (3=soft IRQ, 7=timer IRQ, 11=external IRQ). Otherwise, `code` is set according to the cause of the exception."]
pub type CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CODE_A>;
impl<'a, REG> CODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Instruction fetch was misaligned. Will never fire on RP2350, since the C extension is enabled."]
    #[inline(always)]
    pub fn instr_align(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::INSTR_ALIGN)
    }
    #[doc = "Instruction access fault. Instruction fetch failed a PMP check, or encountered a downstream bus fault, and then passed the point of no speculation."]
    #[inline(always)]
    pub fn instr_fault(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::INSTR_FAULT)
    }
    #[doc = "Illegal instruction was executed (including illegal CSR accesses)"]
    #[inline(always)]
    pub fn illegal_instr(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::ILLEGAL_INSTR)
    }
    #[doc = "Breakpoint. An ebreak instruction was executed when the relevant dcsr.ebreak bit was clear."]
    #[inline(always)]
    pub fn breakpoint(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::BREAKPOINT)
    }
    #[doc = "Load address misaligned. Hazard3 requires natural alignment of all accesses."]
    #[inline(always)]
    pub fn load_align(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::LOAD_ALIGN)
    }
    #[doc = "Load access fault. A load failed a PMP check, or encountered a downstream bus error."]
    #[inline(always)]
    pub fn load_fault(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::LOAD_FAULT)
    }
    #[doc = "Store/AMO address misaligned. Hazard3 requires natural alignment of all accesses."]
    #[inline(always)]
    pub fn store_align(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::STORE_ALIGN)
    }
    #[doc = "Store/AMO access fault. A store/AMO failed a PMP check, or encountered a downstream bus error. Also set if an AMO is attempted on a region that does not support atomics (on RP2350, anything but SRAM)."]
    #[inline(always)]
    pub fn store_fault(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::STORE_FAULT)
    }
    #[doc = "Environment call from U-mode."]
    #[inline(always)]
    pub fn u_ecall(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::U_ECALL)
    }
    #[doc = "Environment call from M-mode."]
    #[inline(always)]
    pub fn m_ecall(self) -> &'a mut crate::W<REG> {
        self.variant(CODE_A::M_ECALL)
    }
}
#[doc = "Field `INTERRUPT` reader - If 1, the trap was caused by an interrupt. If 0, it was caused by an exception."]
pub type INTERRUPT_R = crate::BitReader;
#[doc = "Field `INTERRUPT` writer - If 1, the trap was caused by an interrupt. If 0, it was caused by an exception."]
pub type INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - If `interrupt` is set, `code` indicates the index of the bit in mip that caused the trap (3=soft IRQ, 7=timer IRQ, 11=external IRQ). Otherwise, `code` is set according to the cause of the exception."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - If 1, the trap was caused by an interrupt. If 0, it was caused by an exception."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - If `interrupt` is set, `code` indicates the index of the bit in mip that caused the trap (3=soft IRQ, 7=timer IRQ, 11=external IRQ). Otherwise, `code` is set according to the cause of the exception."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<MCAUSE_SPEC> {
        CODE_W::new(self, 0)
    }
    #[doc = "Bit 31 - If 1, the trap was caused by an interrupt. If 0, it was caused by an exception."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<MCAUSE_SPEC> {
        INTERRUPT_W::new(self, 31)
    }
}
#[doc = "Machine trap cause. Set when entering a trap to indicate the reason for the trap. Readable and writable by software.  

You can [`read`](crate::Reg::read) this register and get [`mcause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCAUSE_SPEC;
impl crate::RegisterSpec for MCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcause::R`](R) reader structure"]
impl crate::Readable for MCAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcause::W`](W) writer structure"]
impl crate::Writable for MCAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAUSE to value 0"]
impl crate::Resettable for MCAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
