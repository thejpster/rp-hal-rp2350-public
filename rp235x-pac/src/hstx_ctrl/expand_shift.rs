#[doc = "Register `EXPAND_SHIFT` reader"]
pub type R = crate::R<EXPAND_SHIFT_SPEC>;
#[doc = "Register `EXPAND_SHIFT` writer"]
pub type W = crate::W<EXPAND_SHIFT_SPEC>;
#[doc = "Field `RAW_SHIFT` reader - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
pub type RAW_SHIFT_R = crate::FieldReader;
#[doc = "Field `RAW_SHIFT` writer - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
pub type RAW_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RAW_N_SHIFTS` reader - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
pub type RAW_N_SHIFTS_R = crate::FieldReader;
#[doc = "Field `RAW_N_SHIFTS` writer - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
pub type RAW_N_SHIFTS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENC_SHIFT` reader - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
pub type ENC_SHIFT_R = crate::FieldReader;
#[doc = "Field `ENC_SHIFT` writer - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
pub type ENC_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENC_N_SHIFTS` reader - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
pub type ENC_N_SHIFTS_R = crate::FieldReader;
#[doc = "Field `ENC_N_SHIFTS` writer - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
pub type ENC_N_SHIFTS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
    #[inline(always)]
    pub fn raw_shift(&self) -> RAW_SHIFT_R {
        RAW_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub fn raw_n_shifts(&self) -> RAW_N_SHIFTS_R {
        RAW_N_SHIFTS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
    #[inline(always)]
    pub fn enc_shift(&self) -> ENC_SHIFT_R {
        ENC_SHIFT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub fn enc_n_shifts(&self) -> ENC_N_SHIFTS_R {
        ENC_N_SHIFTS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
    #[inline(always)]
    #[must_use]
    pub fn raw_shift(&mut self) -> RAW_SHIFT_W<EXPAND_SHIFT_SPEC> {
        RAW_SHIFT_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
    #[inline(always)]
    #[must_use]
    pub fn raw_n_shifts(&mut self) -> RAW_N_SHIFTS_W<EXPAND_SHIFT_SPEC> {
        RAW_N_SHIFTS_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
    #[inline(always)]
    #[must_use]
    pub fn enc_shift(&mut self) -> ENC_SHIFT_W<EXPAND_SHIFT_SPEC> {
        ENC_SHIFT_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
    #[inline(always)]
    #[must_use]
    pub fn enc_n_shifts(&mut self) -> ENC_N_SHIFTS_W<EXPAND_SHIFT_SPEC> {
        ENC_N_SHIFTS_W::new(self, 24)
    }
}
#[doc = "Configure the optional shifter inside the command expander  

You can [`read`](crate::Reg::read) this register and get [`expand_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXPAND_SHIFT_SPEC;
impl crate::RegisterSpec for EXPAND_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`expand_shift::R`](R) reader structure"]
impl crate::Readable for EXPAND_SHIFT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`expand_shift::W`](W) writer structure"]
impl crate::Writable for EXPAND_SHIFT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXPAND_SHIFT to value 0x0100_0100"]
impl crate::Resettable for EXPAND_SHIFT_SPEC {
    const RESET_VALUE: u32 = 0x0100_0100;
}
