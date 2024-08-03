#[doc = "Register `FP_CTRL` reader"]
pub type R = crate::R<FP_CTRL_SPEC>;
#[doc = "Register `FP_CTRL` writer"]
pub type W = crate::W<FP_CTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Enables the FPB"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the FPB"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KEY_R = crate::BitReader;
#[doc = "Field `KEY` writer - Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KEY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_CODE_7_4_` reader - Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_7_4__R = crate::FieldReader;
#[doc = "Field `NUM_LIT` reader - Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
pub type NUM_LIT_R = crate::FieldReader;
#[doc = "Field `NUM_CODE_14_12_` reader - Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_14_12__R = crate::FieldReader;
#[doc = "Field `REV` reader - Flash Patch and Breakpoint Unit architecture revision"]
pub type REV_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enables the FPB"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_7_4_(&self) -> NUM_CODE_7_4__R {
        NUM_CODE_7_4__R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_14_12_(&self) -> NUM_CODE_14_12__R {
        NUM_CODE_14_12__R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Flash Patch and Breakpoint Unit architecture revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the FPB"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<FP_CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<FP_CTRL_SPEC> {
        KEY_W::new(self, 1)
    }
}
#[doc = "Provides FPB implementation information, and the global enable for the FPB unit  

You can [`read`](crate::Reg::read) this register and get [`fp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_CTRL_SPEC;
impl crate::RegisterSpec for FP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_ctrl::R`](R) reader structure"]
impl crate::Readable for FP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_ctrl::W`](W) writer structure"]
impl crate::Writable for FP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_CTRL to value 0x6000_5580"]
impl crate::Resettable for FP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x6000_5580;
}
