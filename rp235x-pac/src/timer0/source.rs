#[doc = "Register `SOURCE` reader"]
pub type R = crate::R<SOURCE_SPEC>;
#[doc = "Register `SOURCE` writer"]
pub type W = crate::W<SOURCE_SPEC>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_SYS_A {
    #[doc = "0: `0`"]
    TICK = 0,
    #[doc = "1: `1`"]
    CLK_SYS = 1,
}
impl From<CLK_SYS_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SYS` reader - "]
pub type CLK_SYS_R = crate::BitReader<CLK_SYS_A>;
impl CLK_SYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SYS_A {
        match self.bits {
            false => CLK_SYS_A::TICK,
            true => CLK_SYS_A::CLK_SYS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_tick(&self) -> bool {
        *self == CLK_SYS_A::TICK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clk_sys(&self) -> bool {
        *self == CLK_SYS_A::CLK_SYS
    }
}
#[doc = "Field `CLK_SYS` writer - "]
pub type CLK_SYS_W<'a, REG> = crate::BitWriter<'a, REG, CLK_SYS_A>;
impl<'a, REG> CLK_SYS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tick(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SYS_A::TICK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk_sys(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SYS_A::CLK_SYS)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_sys(&self) -> CLK_SYS_R {
        CLK_SYS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sys(&mut self) -> CLK_SYS_W<SOURCE_SPEC> {
        CLK_SYS_W::new(self, 0)
    }
}
#[doc = "Selects the source for the timer. Defaults to the normal tick configured in the ticks block (typically configured to 1 microsecond). Writing to 1 will ignore the tick and count clk_sys cycles instead.  

You can [`read`](crate::Reg::read) this register and get [`source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOURCE_SPEC;
impl crate::RegisterSpec for SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`source::R`](R) reader structure"]
impl crate::Readable for SOURCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`source::W`](W) writer structure"]
impl crate::Writable for SOURCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOURCE to value 0"]
impl crate::Resettable for SOURCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
