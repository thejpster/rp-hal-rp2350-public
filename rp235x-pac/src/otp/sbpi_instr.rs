#[doc = "Register `SBPI_INSTR` reader"]
pub type R = crate::R<SBPI_INSTR_SPEC>;
#[doc = "Register `SBPI_INSTR` writer"]
pub type W = crate::W<SBPI_INSTR_SPEC>;
#[doc = "Field `SHORT_WDATA` reader - wdata to be used only when payload_size_m1=0"]
pub type SHORT_WDATA_R = crate::FieldReader;
#[doc = "Field `SHORT_WDATA` writer - wdata to be used only when payload_size_m1=0"]
pub type SHORT_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD` reader - "]
pub type CMD_R = crate::FieldReader;
#[doc = "Field `CMD` writer - "]
pub type CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TARGET` reader - Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
pub type TARGET_R = crate::FieldReader;
#[doc = "Field `TARGET` writer - Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
pub type TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAYLOAD_SIZE_M1` reader - Instruction payload size in bytes minus 1"]
pub type PAYLOAD_SIZE_M1_R = crate::FieldReader;
#[doc = "Field `PAYLOAD_SIZE_M1` writer - Instruction payload size in bytes minus 1"]
pub type PAYLOAD_SIZE_M1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HAS_PAYLOAD` reader - Instruction has payload (data to be written or to be read)"]
pub type HAS_PAYLOAD_R = crate::BitReader;
#[doc = "Field `HAS_PAYLOAD` writer - Instruction has payload (data to be written or to be read)"]
pub type HAS_PAYLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IS_WR` reader - Payload type is write"]
pub type IS_WR_R = crate::BitReader;
#[doc = "Field `IS_WR` writer - Payload type is write"]
pub type IS_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEC` writer - Execute instruction"]
pub type EXEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - wdata to be used only when payload_size_m1=0"]
    #[inline(always)]
    pub fn short_wdata(&self) -> SHORT_WDATA_R {
        SHORT_WDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
    #[inline(always)]
    pub fn target(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Instruction payload size in bytes minus 1"]
    #[inline(always)]
    pub fn payload_size_m1(&self) -> PAYLOAD_SIZE_M1_R {
        PAYLOAD_SIZE_M1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Instruction has payload (data to be written or to be read)"]
    #[inline(always)]
    pub fn has_payload(&self) -> HAS_PAYLOAD_R {
        HAS_PAYLOAD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Payload type is write"]
    #[inline(always)]
    pub fn is_wr(&self) -> IS_WR_R {
        IS_WR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - wdata to be used only when payload_size_m1=0"]
    #[inline(always)]
    #[must_use]
    pub fn short_wdata(&mut self) -> SHORT_WDATA_W<SBPI_INSTR_SPEC> {
        SHORT_WDATA_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<SBPI_INSTR_SPEC> {
        CMD_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
    #[inline(always)]
    #[must_use]
    pub fn target(&mut self) -> TARGET_W<SBPI_INSTR_SPEC> {
        TARGET_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Instruction payload size in bytes minus 1"]
    #[inline(always)]
    #[must_use]
    pub fn payload_size_m1(&mut self) -> PAYLOAD_SIZE_M1_W<SBPI_INSTR_SPEC> {
        PAYLOAD_SIZE_M1_W::new(self, 24)
    }
    #[doc = "Bit 28 - Instruction has payload (data to be written or to be read)"]
    #[inline(always)]
    #[must_use]
    pub fn has_payload(&mut self) -> HAS_PAYLOAD_W<SBPI_INSTR_SPEC> {
        HAS_PAYLOAD_W::new(self, 28)
    }
    #[doc = "Bit 29 - Payload type is write"]
    #[inline(always)]
    #[must_use]
    pub fn is_wr(&mut self) -> IS_WR_W<SBPI_INSTR_SPEC> {
        IS_WR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Execute instruction"]
    #[inline(always)]
    #[must_use]
    pub fn exec(&mut self) -> EXEC_W<SBPI_INSTR_SPEC> {
        EXEC_W::new(self, 30)
    }
}
#[doc = "Dispatch instructions to the SBPI interface, used for programming the OTP fuses.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_instr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_instr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_INSTR_SPEC;
impl crate::RegisterSpec for SBPI_INSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_instr::R`](R) reader structure"]
impl crate::Readable for SBPI_INSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbpi_instr::W`](W) writer structure"]
impl crate::Writable for SBPI_INSTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBPI_INSTR to value 0"]
impl crate::Resettable for SBPI_INSTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
