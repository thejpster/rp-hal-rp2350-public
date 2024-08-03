#[doc = "Register `CTRL_STATUS` reader"]
pub type R = crate::R<CTRL_STATUS_SPEC>;
#[doc = "Register `CTRL_STATUS` writer"]
pub type W = crate::W<CTRL_STATUS_SPEC>;
#[doc = "Field `TRACE_CAPTURE_FIFO_FLUSH` reader - flushes trace capture fifo"]
pub type TRACE_CAPTURE_FIFO_FLUSH_R = crate::BitReader;
#[doc = "Field `TRACE_CAPTURE_FIFO_FLUSH` writer - flushes trace capture fifo"]
pub type TRACE_CAPTURE_FIFO_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_CAPTURE_FIFO_OVERFLOW` reader - "]
pub type TRACE_CAPTURE_FIFO_OVERFLOW_R = crate::BitReader;
#[doc = "Field `TRACE_CAPTURE_FIFO_OVERFLOW` writer - "]
pub type TRACE_CAPTURE_FIFO_OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - flushes trace capture fifo"]
    #[inline(always)]
    pub fn trace_capture_fifo_flush(&self) -> TRACE_CAPTURE_FIFO_FLUSH_R {
        TRACE_CAPTURE_FIFO_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trace_capture_fifo_overflow(&self) -> TRACE_CAPTURE_FIFO_OVERFLOW_R {
        TRACE_CAPTURE_FIFO_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - flushes trace capture fifo"]
    #[inline(always)]
    #[must_use]
    pub fn trace_capture_fifo_flush(&mut self) -> TRACE_CAPTURE_FIFO_FLUSH_W<CTRL_STATUS_SPEC> {
        TRACE_CAPTURE_FIFO_FLUSH_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn trace_capture_fifo_overflow(
        &mut self,
    ) -> TRACE_CAPTURE_FIFO_OVERFLOW_W<CTRL_STATUS_SPEC> {
        TRACE_CAPTURE_FIFO_OVERFLOW_W::new(self, 1)
    }
}
#[doc = "miscellaneous control/status bits  

You can [`read`](crate::Reg::read) this register and get [`ctrl_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_STATUS_SPEC;
impl crate::RegisterSpec for CTRL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_status::R`](R) reader structure"]
impl crate::Readable for CTRL_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_status::W`](W) writer structure"]
impl crate::Writable for CTRL_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_STATUS to value 0x01"]
impl crate::Resettable for CTRL_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
