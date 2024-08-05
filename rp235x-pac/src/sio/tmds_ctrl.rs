#[doc = "Register `TMDS_CTRL` reader"]
pub type R = crate::R<TMDS_CTRL_SPEC>;
#[doc = "Register `TMDS_CTRL` writer"]
pub type W = crate::W<TMDS_CTRL_SPEC>;
#[doc = "Field `L0_ROT` reader - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
pub type L0_ROT_R = crate::FieldReader;
#[doc = "Field `L0_ROT` writer - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
pub type L0_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1_ROT` reader - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
pub type L1_ROT_R = crate::FieldReader;
#[doc = "Field `L1_ROT` writer - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
pub type L1_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L2_ROT` reader - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
pub type L2_ROT_R = crate::FieldReader;
#[doc = "Field `L2_ROT` writer - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
pub type L2_ROT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L0_NBITS` reader - Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L0_NBITS_R = crate::FieldReader;
#[doc = "Field `L0_NBITS` writer - Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L0_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `L1_NBITS` reader - Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L1_NBITS_R = crate::FieldReader;
#[doc = "Field `L1_NBITS` writer - Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L1_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `L2_NBITS` reader - Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L2_NBITS_R = crate::FieldReader;
#[doc = "Field `L2_NBITS` writer - Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
pub type L2_NBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTERLEAVE` reader - Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
pub type INTERLEAVE_R = crate::BitReader;
#[doc = "Field `INTERLEAVE` writer - Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
pub type INTERLEAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIX_SHIFT_A {
    #[doc = "0: Do not shift the colour data register."]
    _0 = 0,
    #[doc = "1: Shift the colour data register by 1 bit"]
    _1 = 1,
    #[doc = "2: Shift the colour data register by 2 bits"]
    _2 = 2,
    #[doc = "3: Shift the colour data register by 4 bits"]
    _4 = 3,
    #[doc = "4: Shift the colour data register by 8 bits"]
    _8 = 4,
    #[doc = "5: Shift the colour data register by 16 bits"]
    _16 = 5,
}
impl From<PIX_SHIFT_A> for u8 {
    #[inline(always)]
    fn from(variant: PIX_SHIFT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIX_SHIFT_A {
    type Ux = u8;
}
impl crate::IsEnum for PIX_SHIFT_A {}
#[doc = "Field `PIX_SHIFT` reader - Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
pub type PIX_SHIFT_R = crate::FieldReader<PIX_SHIFT_A>;
impl PIX_SHIFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIX_SHIFT_A> {
        match self.bits {
            0 => Some(PIX_SHIFT_A::_0),
            1 => Some(PIX_SHIFT_A::_1),
            2 => Some(PIX_SHIFT_A::_2),
            3 => Some(PIX_SHIFT_A::_4),
            4 => Some(PIX_SHIFT_A::_8),
            5 => Some(PIX_SHIFT_A::_16),
            _ => None,
        }
    }
    #[doc = "Do not shift the colour data register."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIX_SHIFT_A::_0
    }
    #[doc = "Shift the colour data register by 1 bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIX_SHIFT_A::_1
    }
    #[doc = "Shift the colour data register by 2 bits"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PIX_SHIFT_A::_2
    }
    #[doc = "Shift the colour data register by 4 bits"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PIX_SHIFT_A::_4
    }
    #[doc = "Shift the colour data register by 8 bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PIX_SHIFT_A::_8
    }
    #[doc = "Shift the colour data register by 16 bits"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PIX_SHIFT_A::_16
    }
}
#[doc = "Field `PIX_SHIFT` writer - Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
pub type PIX_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PIX_SHIFT_A>;
impl<'a, REG> PIX_SHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not shift the colour data register."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_0)
    }
    #[doc = "Shift the colour data register by 1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_1)
    }
    #[doc = "Shift the colour data register by 2 bits"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_2)
    }
    #[doc = "Shift the colour data register by 4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_4)
    }
    #[doc = "Shift the colour data register by 8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_8)
    }
    #[doc = "Shift the colour data register by 16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(PIX_SHIFT_A::_16)
    }
}
#[doc = "Field `PIX2_NOSHIFT` reader - When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
pub type PIX2_NOSHIFT_R = crate::BitReader;
#[doc = "Field `PIX2_NOSHIFT` writer - When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
pub type PIX2_NOSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_BALANCE` writer - Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline."]
pub type CLEAR_BALANCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub fn l0_rot(&self) -> L0_ROT_R {
        L0_ROT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
    #[inline(always)]
    pub fn l1_rot(&self) -> L1_ROT_R {
        L1_ROT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub fn l2_rot(&self) -> L2_ROT_R {
        L2_ROT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn l0_nbits(&self) -> L0_NBITS_R {
        L0_NBITS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn l1_nbits(&self) -> L1_NBITS_R {
        L1_NBITS_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn l2_nbits(&self) -> L2_NBITS_R {
        L2_NBITS_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
    #[inline(always)]
    pub fn interleave(&self) -> INTERLEAVE_R {
        INTERLEAVE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
    #[inline(always)]
    pub fn pix_shift(&self) -> PIX_SHIFT_R {
        PIX_SHIFT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
    #[inline(always)]
    pub fn pix2_noshift(&self) -> PIX2_NOSHIFT_R {
        PIX2_NOSHIFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    #[must_use]
    pub fn l0_rot(&mut self) -> L0_ROT_W<TMDS_CTRL_SPEC> {
        L0_ROT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
    #[inline(always)]
    #[must_use]
    pub fn l1_rot(&mut self) -> L1_ROT_W<TMDS_CTRL_SPEC> {
        L1_ROT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    #[must_use]
    pub fn l2_rot(&mut self) -> L2_ROT_W<TMDS_CTRL_SPEC> {
        L2_ROT_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    #[must_use]
    pub fn l0_nbits(&mut self) -> L0_NBITS_W<TMDS_CTRL_SPEC> {
        L0_NBITS_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    #[must_use]
    pub fn l1_nbits(&mut self) -> L1_NBITS_W<TMDS_CTRL_SPEC> {
        L1_NBITS_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    #[must_use]
    pub fn l2_nbits(&mut self) -> L2_NBITS_W<TMDS_CTRL_SPEC> {
        L2_NBITS_W::new(self, 18)
    }
    #[doc = "Bit 23 - Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
    #[inline(always)]
    #[must_use]
    pub fn interleave(&mut self) -> INTERLEAVE_W<TMDS_CTRL_SPEC> {
        INTERLEAVE_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
    #[inline(always)]
    #[must_use]
    pub fn pix_shift(&mut self) -> PIX_SHIFT_W<TMDS_CTRL_SPEC> {
        PIX_SHIFT_W::new(self, 24)
    }
    #[doc = "Bit 27 - When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
    #[inline(always)]
    #[must_use]
    pub fn pix2_noshift(&mut self) -> PIX2_NOSHIFT_W<TMDS_CTRL_SPEC> {
        PIX2_NOSHIFT_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline."]
    #[inline(always)]
    #[must_use]
    pub fn clear_balance(&mut self) -> CLEAR_BALANCE_W<TMDS_CTRL_SPEC> {
        CLEAR_BALANCE_W::new(self, 28)
    }
}
#[doc = "Control register for TMDS encoder.  

You can [`read`](crate::Reg::read) this register and get [`tmds_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_CTRL_SPEC;
impl crate::RegisterSpec for TMDS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_ctrl::R`](R) reader structure"]
impl crate::Readable for TMDS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmds_ctrl::W`](W) writer structure"]
impl crate::Writable for TMDS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_CTRL to value 0"]
impl crate::Resettable for TMDS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
