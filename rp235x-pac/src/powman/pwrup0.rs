#[doc = "Register `PWRUP0` reader"]
pub type R = crate::R<PWRUP0_SPEC>;
#[doc = "Register `PWRUP0` writer"]
pub type W = crate::W<PWRUP0_SPEC>;
#[doc = "Field `SOURCE` reader - "]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - "]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENABLE` reader - Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRECTION_A {
    #[doc = "0: `0`"]
    LOW_FALLING = 0,
    #[doc = "1: `1`"]
    HIGH_RISING = 1,
}
impl From<DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECTION` reader - "]
pub type DIRECTION_R = crate::BitReader<DIRECTION_A>;
impl DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRECTION_A {
        match self.bits {
            false => DIRECTION_A::LOW_FALLING,
            true => DIRECTION_A::HIGH_RISING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low_falling(&self) -> bool {
        *self == DIRECTION_A::LOW_FALLING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high_rising(&self) -> bool {
        *self == DIRECTION_A::HIGH_RISING
    }
}
#[doc = "Field `DIRECTION` writer - "]
pub type DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG, DIRECTION_A>;
impl<'a, REG> DIRECTION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low_falling(self) -> &'a mut crate::W<REG> {
        self.variant(DIRECTION_A::LOW_FALLING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high_rising(self) -> &'a mut crate::W<REG> {
        self.variant(DIRECTION_A::HIGH_RISING)
    }
}
#[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    LEVEL = 0,
    #[doc = "1: `1`"]
    EDGE = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::LEVEL,
            true => MODE_A::EDGE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MODE_A::LEVEL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MODE_A::EDGE
    }
}
#[doc = "Field `MODE` writer - Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::LEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EDGE)
    }
}
#[doc = "Field `STATUS` reader - Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
pub type STATUS_R = crate::BitReader;
#[doc = "Field `STATUS` writer - Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
pub type STATUS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RAW_STATUS` reader - Value of selected gpio pin (only if enable == 1)"]
pub type RAW_STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub fn raw_status(&self) -> RAW_STATUS_R {
        RAW_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<PWRUP0_SPEC> {
        SOURCE_W::new(self, 0)
    }
    #[doc = "Bit 6 - Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<PWRUP0_SPEC> {
        ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn direction(&mut self) -> DIRECTION_W<PWRUP0_SPEC> {
        DIRECTION_W::new(self, 7)
    }
    #[doc = "Bit 8 - Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<PWRUP0_SPEC> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<PWRUP0_SPEC> {
        STATUS_W::new(self, 9)
    }
}
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high  

You can [`read`](crate::Reg::read) this register and get [`pwrup0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrup0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRUP0_SPEC;
impl crate::RegisterSpec for PWRUP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrup0::R`](R) reader structure"]
impl crate::Readable for PWRUP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrup0::W`](W) writer structure"]
impl crate::Writable for PWRUP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0200;
}
#[doc = "`reset()` method sets PWRUP0 to value 0x3f"]
impl crate::Resettable for PWRUP0_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
