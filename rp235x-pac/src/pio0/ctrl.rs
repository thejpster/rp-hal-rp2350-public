#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SM_ENABLE` reader - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
pub type SM_ENABLE_R = crate::FieldReader;
#[doc = "Field `SM_ENABLE` writer - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
pub type SM_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SM_RESTART` writer - Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution. Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY. The contents of the output shift register and the X/Y scratch registers are not affected."]
pub type SM_RESTART_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV_RESTART` writer - Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep. Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync. Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
pub type CLKDIV_RESTART_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PREV_PIO_MASK` writer - A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
pub type PREV_PIO_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NEXT_PIO_MASK` writer - A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together). Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
pub type NEXT_PIO_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NEXTPREV_SM_ENABLE` writer - Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers. If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence."]
pub type NEXTPREV_SM_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEXTPREV_SM_DISABLE` writer - Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers."]
pub type NEXTPREV_SM_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEXTPREV_CLKDIV_RESTART` writer - Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers."]
pub type NEXTPREV_CLKDIV_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    pub fn sm_enable(&self) -> SM_ENABLE_R {
        SM_ENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    #[must_use]
    pub fn sm_enable(&mut self) -> SM_ENABLE_W<CTRL_SPEC> {
        SM_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution. Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY. The contents of the output shift register and the X/Y scratch registers are not affected."]
    #[inline(always)]
    #[must_use]
    pub fn sm_restart(&mut self) -> SM_RESTART_W<CTRL_SPEC> {
        SM_RESTART_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep. Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync. Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_restart(&mut self) -> CLKDIV_RESTART_W<CTRL_SPEC> {
        CLKDIV_RESTART_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    #[must_use]
    pub fn prev_pio_mask(&mut self) -> PREV_PIO_MASK_W<CTRL_SPEC> {
        PREV_PIO_MASK_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together). Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    #[must_use]
    pub fn next_pio_mask(&mut self) -> NEXT_PIO_MASK_W<CTRL_SPEC> {
        NEXT_PIO_MASK_W::new(self, 20)
    }
    #[doc = "Bit 24 - Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers. If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence."]
    #[inline(always)]
    #[must_use]
    pub fn nextprev_sm_enable(&mut self) -> NEXTPREV_SM_ENABLE_W<CTRL_SPEC> {
        NEXTPREV_SM_ENABLE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers."]
    #[inline(always)]
    #[must_use]
    pub fn nextprev_sm_disable(&mut self) -> NEXTPREV_SM_DISABLE_W<CTRL_SPEC> {
        NEXTPREV_SM_DISABLE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers."]
    #[inline(always)]
    #[must_use]
    pub fn nextprev_clkdiv_restart(&mut self) -> NEXTPREV_CLKDIV_RESTART_W<CTRL_SPEC> {
        NEXTPREV_CLKDIV_RESTART_W::new(self, 26)
    }
}
#[doc = "PIO control register  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
