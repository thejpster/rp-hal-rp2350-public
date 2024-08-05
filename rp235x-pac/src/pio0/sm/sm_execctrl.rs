#[doc = "Register `SM_EXECCTRL` reader"]
pub type R = crate::R<SM_EXECCTRL_SPEC>;
#[doc = "Register `SM_EXECCTRL` writer"]
pub type W = crate::W<SM_EXECCTRL_SPEC>;
#[doc = "Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATUS_N_A {
    #[doc = "0: Index 0-7 of an IRQ flag in this PIO block"]
    IRQ = 0,
    #[doc = "8: Index 0-7 of an IRQ flag in the next lower-numbered PIO block"]
    IRQ_PREVPIO = 8,
    #[doc = "16: Index 0-7 of an IRQ flag in the next higher-numbered PIO block"]
    IRQ_NEXTPIO = 16,
}
impl From<STATUS_N_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_N_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATUS_N_A {
    type Ux = u8;
}
impl crate::IsEnum for STATUS_N_A {}
#[doc = "Field `STATUS_N` reader - Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
pub type STATUS_N_R = crate::FieldReader<STATUS_N_A>;
impl STATUS_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STATUS_N_A> {
        match self.bits {
            0 => Some(STATUS_N_A::IRQ),
            8 => Some(STATUS_N_A::IRQ_PREVPIO),
            16 => Some(STATUS_N_A::IRQ_NEXTPIO),
            _ => None,
        }
    }
    #[doc = "Index 0-7 of an IRQ flag in this PIO block"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == STATUS_N_A::IRQ
    }
    #[doc = "Index 0-7 of an IRQ flag in the next lower-numbered PIO block"]
    #[inline(always)]
    pub fn is_irq_prevpio(&self) -> bool {
        *self == STATUS_N_A::IRQ_PREVPIO
    }
    #[doc = "Index 0-7 of an IRQ flag in the next higher-numbered PIO block"]
    #[inline(always)]
    pub fn is_irq_nextpio(&self) -> bool {
        *self == STATUS_N_A::IRQ_NEXTPIO
    }
}
#[doc = "Field `STATUS_N` writer - Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
pub type STATUS_N_W<'a, REG> = crate::FieldWriter<'a, REG, 5, STATUS_N_A>;
impl<'a, REG> STATUS_N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Index 0-7 of an IRQ flag in this PIO block"]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_N_A::IRQ)
    }
    #[doc = "Index 0-7 of an IRQ flag in the next lower-numbered PIO block"]
    #[inline(always)]
    pub fn irq_prevpio(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_N_A::IRQ_PREVPIO)
    }
    #[doc = "Index 0-7 of an IRQ flag in the next higher-numbered PIO block"]
    #[inline(always)]
    pub fn irq_nextpio(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_N_A::IRQ_NEXTPIO)
    }
}
#[doc = "Comparison used for the MOV x, STATUS instruction.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATUS_SEL_A {
    #[doc = "0: All-ones if TX FIFO level &lt; N, otherwise all-zeroes"]
    TXLEVEL = 0,
    #[doc = "1: All-ones if RX FIFO level &lt; N, otherwise all-zeroes"]
    RXLEVEL = 1,
    #[doc = "2: All-ones if the indexed IRQ flag is raised, otherwise all-zeroes"]
    IRQ = 2,
}
impl From<STATUS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATUS_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for STATUS_SEL_A {}
#[doc = "Field `STATUS_SEL` reader - Comparison used for the MOV x, STATUS instruction."]
pub type STATUS_SEL_R = crate::FieldReader<STATUS_SEL_A>;
impl STATUS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STATUS_SEL_A> {
        match self.bits {
            0 => Some(STATUS_SEL_A::TXLEVEL),
            1 => Some(STATUS_SEL_A::RXLEVEL),
            2 => Some(STATUS_SEL_A::IRQ),
            _ => None,
        }
    }
    #[doc = "All-ones if TX FIFO level &lt; N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn is_txlevel(&self) -> bool {
        *self == STATUS_SEL_A::TXLEVEL
    }
    #[doc = "All-ones if RX FIFO level &lt; N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn is_rxlevel(&self) -> bool {
        *self == STATUS_SEL_A::RXLEVEL
    }
    #[doc = "All-ones if the indexed IRQ flag is raised, otherwise all-zeroes"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == STATUS_SEL_A::IRQ
    }
}
#[doc = "Field `STATUS_SEL` writer - Comparison used for the MOV x, STATUS instruction."]
pub type STATUS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STATUS_SEL_A>;
impl<'a, REG> STATUS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All-ones if TX FIFO level &lt; N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn txlevel(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_SEL_A::TXLEVEL)
    }
    #[doc = "All-ones if RX FIFO level &lt; N, otherwise all-zeroes"]
    #[inline(always)]
    pub fn rxlevel(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_SEL_A::RXLEVEL)
    }
    #[doc = "All-ones if the indexed IRQ flag is raised, otherwise all-zeroes"]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(STATUS_SEL_A::IRQ)
    }
}
#[doc = "Field `WRAP_BOTTOM` reader - After reaching wrap_top, execution is wrapped to this address."]
pub type WRAP_BOTTOM_R = crate::FieldReader;
#[doc = "Field `WRAP_BOTTOM` writer - After reaching wrap_top, execution is wrapped to this address."]
pub type WRAP_BOTTOM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRAP_TOP` reader - After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
pub type WRAP_TOP_R = crate::FieldReader;
#[doc = "Field `WRAP_TOP` writer - After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
pub type WRAP_TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUT_STICKY` reader - Continuously assert the most recent OUT/SET to the pins"]
pub type OUT_STICKY_R = crate::BitReader;
#[doc = "Field `OUT_STICKY` writer - Continuously assert the most recent OUT/SET to the pins"]
pub type OUT_STICKY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINE_OUT_EN` reader - If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 &lt; SM1 &lt; ...)"]
pub type INLINE_OUT_EN_R = crate::BitReader;
#[doc = "Field `INLINE_OUT_EN` writer - If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 &lt; SM1 &lt; ...)"]
pub type INLINE_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EN_SEL` reader - Which data bit to use for inline OUT enable"]
pub type OUT_EN_SEL_R = crate::FieldReader;
#[doc = "Field `OUT_EN_SEL` writer - Which data bit to use for inline OUT enable"]
pub type OUT_EN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JMP_PIN` reader - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
pub type JMP_PIN_R = crate::FieldReader;
#[doc = "Field `JMP_PIN` writer - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
pub type JMP_PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SIDE_PINDIR` reader - If 1, side-set data is asserted to pin directions, instead of pin values"]
pub type SIDE_PINDIR_R = crate::BitReader;
#[doc = "Field `SIDE_PINDIR` writer - If 1, side-set data is asserted to pin directions, instead of pin values"]
pub type SIDE_PINDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIDE_EN` reader - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
pub type SIDE_EN_R = crate::BitReader;
#[doc = "Field `SIDE_EN` writer - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
pub type SIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEC_STALLED` reader - If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
pub type EXEC_STALLED_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
    #[inline(always)]
    pub fn status_n(&self) -> STATUS_N_R {
        STATUS_N_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    pub fn status_sel(&self) -> STATUS_SEL_R {
        STATUS_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:11 - After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    pub fn wrap_bottom(&self) -> WRAP_BOTTOM_R {
        WRAP_BOTTOM_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    pub fn wrap_top(&self) -> WRAP_TOP_R {
        WRAP_TOP_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn out_sticky(&self) -> OUT_STICKY_R {
        OUT_STICKY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 &lt; SM1 &lt; ...)"]
    #[inline(always)]
    pub fn inline_out_en(&self) -> INLINE_OUT_EN_R {
        INLINE_OUT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Which data bit to use for inline OUT enable"]
    #[inline(always)]
    pub fn out_en_sel(&self) -> OUT_EN_SEL_R {
        OUT_EN_SEL_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    pub fn jmp_pin(&self) -> JMP_PIN_R {
        JMP_PIN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    pub fn side_pindir(&self) -> SIDE_PINDIR_R {
        SIDE_PINDIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    pub fn side_en(&self) -> SIDE_EN_R {
        SIDE_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
    #[inline(always)]
    pub fn exec_stalled(&self) -> EXEC_STALLED_R {
        EXEC_STALLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
    #[inline(always)]
    #[must_use]
    pub fn status_n(&mut self) -> STATUS_N_W<SM_EXECCTRL_SPEC> {
        STATUS_N_W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    #[must_use]
    pub fn status_sel(&mut self) -> STATUS_SEL_W<SM_EXECCTRL_SPEC> {
        STATUS_SEL_W::new(self, 5)
    }
    #[doc = "Bits 7:11 - After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    #[must_use]
    pub fn wrap_bottom(&mut self) -> WRAP_BOTTOM_W<SM_EXECCTRL_SPEC> {
        WRAP_BOTTOM_W::new(self, 7)
    }
    #[doc = "Bits 12:16 - After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    #[must_use]
    pub fn wrap_top(&mut self) -> WRAP_TOP_W<SM_EXECCTRL_SPEC> {
        WRAP_TOP_W::new(self, 12)
    }
    #[doc = "Bit 17 - Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    #[must_use]
    pub fn out_sticky(&mut self) -> OUT_STICKY_W<SM_EXECCTRL_SPEC> {
        OUT_STICKY_W::new(self, 17)
    }
    #[doc = "Bit 18 - If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 &lt; SM1 &lt; ...)"]
    #[inline(always)]
    #[must_use]
    pub fn inline_out_en(&mut self) -> INLINE_OUT_EN_W<SM_EXECCTRL_SPEC> {
        INLINE_OUT_EN_W::new(self, 18)
    }
    #[doc = "Bits 19:23 - Which data bit to use for inline OUT enable"]
    #[inline(always)]
    #[must_use]
    pub fn out_en_sel(&mut self) -> OUT_EN_SEL_W<SM_EXECCTRL_SPEC> {
        OUT_EN_SEL_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    #[must_use]
    pub fn jmp_pin(&mut self) -> JMP_PIN_W<SM_EXECCTRL_SPEC> {
        JMP_PIN_W::new(self, 24)
    }
    #[doc = "Bit 29 - If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    #[must_use]
    pub fn side_pindir(&mut self) -> SIDE_PINDIR_W<SM_EXECCTRL_SPEC> {
        SIDE_PINDIR_W::new(self, 29)
    }
    #[doc = "Bit 30 - If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn side_en(&mut self) -> SIDE_EN_W<SM_EXECCTRL_SPEC> {
        SIDE_EN_W::new(self, 30)
    }
}
#[doc = "Execution/behavioural settings for state machine 0  

You can [`read`](crate::Reg::read) this register and get [`sm_execctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sm_execctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_EXECCTRL_SPEC;
impl crate::RegisterSpec for SM_EXECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_execctrl::R`](R) reader structure"]
impl crate::Readable for SM_EXECCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_execctrl::W`](W) writer structure"]
impl crate::Writable for SM_EXECCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SM_EXECCTRL to value 0x0001_f000"]
impl crate::Resettable for SM_EXECCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_f000;
}
