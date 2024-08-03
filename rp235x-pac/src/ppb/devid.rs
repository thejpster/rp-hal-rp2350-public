#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DEVID_SPEC>;
#[doc = "Field `EXTMUXNUM` reader - Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present. This value of this bit depends on the Verilog define EXTMUXNUM that you must change accordingly."]
pub type EXTMUXNUM_R = crate::FieldReader;
#[doc = "Field `NUMTRIG` reader - Number of ECT triggers available."]
pub type NUMTRIG_R = crate::FieldReader;
#[doc = "Field `NUMCH` reader - Number of ECT channels available"]
pub type NUMCH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present. This value of this bit depends on the Verilog define EXTMUXNUM that you must change accordingly."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> EXTMUXNUM_R {
        EXTMUXNUM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Number of ECT triggers available."]
    #[inline(always)]
    pub fn numtrig(&self) -> NUMTRIG_R {
        NUMTRIG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of ECT channels available"]
    #[inline(always)]
    pub fn numch(&self) -> NUMCH_R {
        NUMCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Device Configuration register  

You can [`read`](crate::Reg::read) this register and get [`devid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DEVID_SPEC {}
#[doc = "`reset()` method sets DEVID to value 0x0004_0800"]
impl crate::Resettable for DEVID_SPEC {
    const RESET_VALUE: u32 = 0x0004_0800;
}
