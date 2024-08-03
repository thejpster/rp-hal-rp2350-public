#[doc = "Register `CLK_GPOUT1_DIV` reader"]
pub type R = crate::R<CLK_GPOUT1_DIV_SPEC>;
#[doc = "Register `CLK_GPOUT1_DIV` writer"]
pub type W = crate::W<CLK_GPOUT1_DIV_SPEC>;
#[doc = "Field `FRAC` reader - Fractional component of the divisor, can be changed on-the-fly"]
pub type FRAC_R = crate::FieldReader<u16>;
#[doc = "Field `FRAC` writer - Fractional component of the divisor, can be changed on-the-fly"]
pub type FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INT` reader - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
pub type INT_R = crate::FieldReader<u16>;
#[doc = "Field `INT` writer - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<CLK_GPOUT1_DIV_SPEC> {
        FRAC_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<CLK_GPOUT1_DIV_SPEC> {
        INT_W::new(self, 16)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`clk_gpout1_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gpout1_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GPOUT1_DIV_SPEC;
impl crate::RegisterSpec for CLK_GPOUT1_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gpout1_div::R`](R) reader structure"]
impl crate::Readable for CLK_GPOUT1_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gpout1_div::W`](W) writer structure"]
impl crate::Writable for CLK_GPOUT1_DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_GPOUT1_DIV to value 0x0001_0000"]
impl crate::Resettable for CLK_GPOUT1_DIV_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
