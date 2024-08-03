#[doc = "Register `ITM_TCR` reader"]
pub type R = crate::R<ITM_TCR_SPEC>;
#[doc = "Register `ITM_TCR` writer"]
pub type W = crate::W<ITM_TCR_SPEC>;
#[doc = "Field `ITMENA` reader - Enables the ITM"]
pub type ITMENA_R = crate::BitReader;
#[doc = "Field `ITMENA` writer - Enables the ITM"]
pub type ITMENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENA` reader - Enables Local timestamp generation"]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - Enables Local timestamp generation"]
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCENA` reader - Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SYNCENA_R = crate::BitReader;
#[doc = "Field `SYNCENA` writer - Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SYNCENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENA` reader - Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TXENA_R = crate::BitReader;
#[doc = "Field `TXENA` writer - Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TXENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWOENA` reader - Enables asynchronous clocking of the timestamp counter"]
pub type SWOENA_R = crate::BitReader;
#[doc = "Field `SWOENA` writer - Enables asynchronous clocking of the timestamp counter"]
pub type SWOENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLENA` reader - Stall the PE to guarantee delivery of Data Trace packets."]
pub type STALLENA_R = crate::BitReader;
#[doc = "Field `STALLENA` writer - Stall the PE to guarantee delivery of Data Trace packets."]
pub type STALLENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPRESCALE` reader - Local timestamp prescaler, used with the trace packet reference clock"]
pub type TSPRESCALE_R = crate::FieldReader;
#[doc = "Field `TSPRESCALE` writer - Local timestamp prescaler, used with the trace packet reference clock"]
pub type TSPRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTSFREQ` reader - Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GTSFREQ_R = crate::FieldReader;
#[doc = "Field `GTSFREQ` writer - Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GTSFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRACEBUSID` reader - Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TRACEBUSID_R = crate::FieldReader;
#[doc = "Field `TRACEBUSID` writer - Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TRACEBUSID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BUSY` reader - Indicates whether the ITM is currently processing events"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enables the ITM"]
    #[inline(always)]
    pub fn itmena(&self) -> ITMENA_R {
        ITMENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables Local timestamp generation"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    pub fn syncena(&self) -> SYNCENA_R {
        SYNCENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    pub fn txena(&self) -> TXENA_R {
        TXENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    pub fn swoena(&self) -> SWOENA_R {
        SWOENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    pub fn stallena(&self) -> STALLENA_R {
        STALLENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TSPRESCALE_R {
        TSPRESCALE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    pub fn gtsfreq(&self) -> GTSFREQ_R {
        GTSFREQ_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    pub fn tracebusid(&self) -> TRACEBUSID_R {
        TRACEBUSID_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Indicates whether the ITM is currently processing events"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the ITM"]
    #[inline(always)]
    #[must_use]
    pub fn itmena(&mut self) -> ITMENA_W<ITM_TCR_SPEC> {
        ITMENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables Local timestamp generation"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<ITM_TCR_SPEC> {
        TSENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn syncena(&mut self) -> SYNCENA_W<ITM_TCR_SPEC> {
        SYNCENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TXENA_W<ITM_TCR_SPEC> {
        TXENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    #[must_use]
    pub fn swoena(&mut self) -> SWOENA_W<ITM_TCR_SPEC> {
        SWOENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    #[must_use]
    pub fn stallena(&mut self) -> STALLENA_W<ITM_TCR_SPEC> {
        STALLENA_W::new(self, 5)
    }
    #[doc = "Bits 8:9 - Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn tsprescale(&mut self) -> TSPRESCALE_W<ITM_TCR_SPEC> {
        TSPRESCALE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    #[must_use]
    pub fn gtsfreq(&mut self) -> GTSFREQ_W<ITM_TCR_SPEC> {
        GTSFREQ_W::new(self, 10)
    }
    #[doc = "Bits 16:22 - Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    #[must_use]
    pub fn tracebusid(&mut self) -> TRACEBUSID_W<ITM_TCR_SPEC> {
        TRACEBUSID_W::new(self, 16)
    }
}
#[doc = "Configures and controls transfers through the ITM interface  

You can [`read`](crate::Reg::read) this register and get [`itm_tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_TCR_SPEC;
impl crate::RegisterSpec for ITM_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_tcr::R`](R) reader structure"]
impl crate::Readable for ITM_TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_tcr::W`](W) writer structure"]
impl crate::Writable for ITM_TCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_TCR to value 0"]
impl crate::Resettable for ITM_TCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
