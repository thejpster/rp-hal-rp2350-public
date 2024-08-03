#[doc = "Register `BIT5` reader"]
pub type R = crate::R<BIT5_SPEC>;
#[doc = "Register `BIT5` writer"]
pub type W = crate::W<BIT5_SPEC>;
#[doc = "Field `SEL_P` reader - Shift register data bit select for the first half of the HSTX clock cycle"]
pub type SEL_P_R = crate::FieldReader;
#[doc = "Field `SEL_P` writer - Shift register data bit select for the first half of the HSTX clock cycle"]
pub type SEL_P_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SEL_N` reader - Shift register data bit select for the second half of the HSTX clock cycle"]
pub type SEL_N_R = crate::FieldReader;
#[doc = "Field `SEL_N` writer - Shift register data bit select for the second half of the HSTX clock cycle"]
pub type SEL_N_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INV` reader - Invert this data output (logical NOT)"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invert this data output (logical NOT)"]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK` reader - Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
pub type CLK_R = crate::BitReader;
#[doc = "Field `CLK` writer - Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
pub type CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn sel_p(&self) -> SEL_P_R {
        SEL_P_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn sel_n(&self) -> SEL_N_R {
        SEL_N_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn sel_p(&mut self) -> SEL_P_W<BIT5_SPEC> {
        SEL_P_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn sel_n(&mut self) -> SEL_N_W<BIT5_SPEC> {
        SEL_N_W::new(self, 8)
    }
    #[doc = "Bit 16 - Invert this data output (logical NOT)"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<BIT5_SPEC> {
        INV_W::new(self, 16)
    }
    #[doc = "Bit 17 - Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<BIT5_SPEC> {
        CLK_W::new(self, 17)
    }
}
#[doc = "Data control register for output bit 5  

You can [`read`](crate::Reg::read) this register and get [`bit5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIT5_SPEC;
impl crate::RegisterSpec for BIT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bit5::R`](R) reader structure"]
impl crate::Readable for BIT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bit5::W`](W) writer structure"]
impl crate::Writable for BIT5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIT5 to value 0"]
impl crate::Resettable for BIT5_SPEC {
    const RESET_VALUE: u32 = 0;
}
