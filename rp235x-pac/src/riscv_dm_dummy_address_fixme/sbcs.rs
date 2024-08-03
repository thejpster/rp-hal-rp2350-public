#[doc = "Register `SBCS` reader"]
pub type R = crate::R<SBCS_SPEC>;
#[doc = "Register `SBCS` writer"]
pub type W = crate::W<SBCS_SPEC>;
#[doc = "Field `SBACCESS8` reader - 1 when 8-bit system bus accesses are supported."]
pub type SBACCESS8_R = crate::BitReader;
#[doc = "Field `SBACCESS16` reader - 1 when 16-bit system bus accesses are supported."]
pub type SBACCESS16_R = crate::BitReader;
#[doc = "Field `SBACCESS32` reader - 1 when 32-bit system bus accesses are supported."]
pub type SBACCESS32_R = crate::BitReader;
#[doc = "Field `SBACCESS64` reader - 1 when 64-bit system bus accesses are supported."]
pub type SBACCESS64_R = crate::BitReader;
#[doc = "Field `SBACCESS128` reader - 1 when 128-bit system bus accesses are supported."]
pub type SBACCESS128_R = crate::BitReader;
#[doc = "Field `SBASIZE` reader - Width of system bus addresses in bits. (0 indicates there is no bus access support.)"]
pub type SBASIZE_R = crate::FieldReader;
#[doc = "Field `SBERROR` reader - When the Debug Module’s system bus master encounters an error, this field gets set. The bits in this field remain set until they are cleared by writing 1 to them. While this field is non-zero, no more system bus accesses can be initiated by the Debug Module.  

 An implementation may report “Other” (7) for any error condition. (Hazard3 does not use this value for any errors.)  

 0: There was no bus error.  

 1: There was a timeout.  

 2: A bad address was accessed.  

 3: There was an alignment error.  

 4: An access of unsupported size was requested.  

 7: Other.  

 Hazard3 raises an alignment error for any non-naturally-aligned bus transfer which would otherwise be a valid transfer."]
pub type SBERROR_R = crate::FieldReader;
#[doc = "Field `SBERROR` writer - When the Debug Module’s system bus master encounters an error, this field gets set. The bits in this field remain set until they are cleared by writing 1 to them. While this field is non-zero, no more system bus accesses can be initiated by the Debug Module.  

 An implementation may report “Other” (7) for any error condition. (Hazard3 does not use this value for any errors.)  

 0: There was no bus error.  

 1: There was a timeout.  

 2: A bad address was accessed.  

 3: There was an alignment error.  

 4: An access of unsupported size was requested.  

 7: Other.  

 Hazard3 raises an alignment error for any non-naturally-aligned bus transfer which would otherwise be a valid transfer."]
pub type SBERROR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SBREADONDATA` reader - When 1, every read from sbdata0 automatically triggers a system bus read at the (possibly auto- incremented) address."]
pub type SBREADONDATA_R = crate::BitReader;
#[doc = "Field `SBREADONDATA` writer - When 1, every read from sbdata0 automatically triggers a system bus read at the (possibly auto- incremented) address."]
pub type SBREADONDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBAUTOINCREMENT` reader - When 1, sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
pub type SBAUTOINCREMENT_R = crate::BitReader;
#[doc = "Field `SBAUTOINCREMENT` writer - When 1, sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
pub type SBAUTOINCREMENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBACCESS` reader - Select the access size to use for system bus accesses.  

 0: 8-bit  

 1: 16-bit  

 2: 32-bit  

 3: 64-bit  

 4: 128-bit  

 If sbaccess has an unsupported value when the DM starts a bus access, the access is not per formed and sberror is set to 4. (On Hazard3 the supported values are 8-bit, 16-bit and 32-bit.)"]
pub type SBACCESS_R = crate::FieldReader;
#[doc = "Field `SBACCESS` writer - Select the access size to use for system bus accesses.  

 0: 8-bit  

 1: 16-bit  

 2: 32-bit  

 3: 64-bit  

 4: 128-bit  

 If sbaccess has an unsupported value when the DM starts a bus access, the access is not per formed and sberror is set to 4. (On Hazard3 the supported values are 8-bit, 16-bit and 32-bit.)"]
pub type SBACCESS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SBREADONADDR` reader - When 1, every write to sbaddress0 automatically triggers a system bus read at the new address."]
pub type SBREADONADDR_R = crate::BitReader;
#[doc = "Field `SBREADONADDR` writer - When 1, every write to sbaddress0 automatically triggers a system bus read at the new address."]
pub type SBREADONADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBBUSY` reader - When 1, indicates the system bus master is busy. (Whether the system bus itself is busy is related, but not the same thing.) This bit goes high immediately when a read or write is requested for any reason, and does not go low until the access is fully completed.  

 Writes to sbcs while sbbusy is high result in undefined behavior. A debugger must not write to sbcs until it reads sbbusy as 0."]
pub type SBBUSY_R = crate::BitReader;
#[doc = "Field `SBBUSYERROR` reader - Set when the debugger attempts to read data while a read is in progress, or when the debugger initiates a new access while one is already in progress (while sbbusy is set). It remains set until it’s explicitly cleared by the debugger.  

 While this field is set, no more system bus accesses can be initiated by the Debug Module."]
pub type SBBUSYERROR_R = crate::BitReader;
#[doc = "Field `SBBUSYERROR` writer - Set when the debugger attempts to read data while a read is in progress, or when the debugger initiates a new access while one is already in progress (while sbbusy is set). It remains set until it’s explicitly cleared by the debugger.  

 While this field is set, no more system bus accesses can be initiated by the Debug Module."]
pub type SBBUSYERROR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SBVERSION` reader - 1: The System Bus interface conforms to version 0.13.2 of the RISC-V debug spec."]
pub type SBVERSION_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 1 when 8-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn sbaccess8(&self) -> SBACCESS8_R {
        SBACCESS8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 when 16-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn sbaccess16(&self) -> SBACCESS16_R {
        SBACCESS16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 when 32-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn sbaccess32(&self) -> SBACCESS32_R {
        SBACCESS32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 when 64-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn sbaccess64(&self) -> SBACCESS64_R {
        SBACCESS64_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 when 128-bit system bus accesses are supported."]
    #[inline(always)]
    pub fn sbaccess128(&self) -> SBACCESS128_R {
        SBACCESS128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Width of system bus addresses in bits. (0 indicates there is no bus access support.)"]
    #[inline(always)]
    pub fn sbasize(&self) -> SBASIZE_R {
        SBASIZE_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 12:14 - When the Debug Module’s system bus master encounters an error, this field gets set. The bits in this field remain set until they are cleared by writing 1 to them. While this field is non-zero, no more system bus accesses can be initiated by the Debug Module.  

 An implementation may report “Other” (7) for any error condition. (Hazard3 does not use this value for any errors.)  

 0: There was no bus error.  

 1: There was a timeout.  

 2: A bad address was accessed.  

 3: There was an alignment error.  

 4: An access of unsupported size was requested.  

 7: Other.  

 Hazard3 raises an alignment error for any non-naturally-aligned bus transfer which would otherwise be a valid transfer."]
    #[inline(always)]
    pub fn sberror(&self) -> SBERROR_R {
        SBERROR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - When 1, every read from sbdata0 automatically triggers a system bus read at the (possibly auto- incremented) address."]
    #[inline(always)]
    pub fn sbreadondata(&self) -> SBREADONDATA_R {
        SBREADONDATA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
    #[inline(always)]
    pub fn sbautoincrement(&self) -> SBAUTOINCREMENT_R {
        SBAUTOINCREMENT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Select the access size to use for system bus accesses.  

 0: 8-bit  

 1: 16-bit  

 2: 32-bit  

 3: 64-bit  

 4: 128-bit  

 If sbaccess has an unsupported value when the DM starts a bus access, the access is not per formed and sberror is set to 4. (On Hazard3 the supported values are 8-bit, 16-bit and 32-bit.)"]
    #[inline(always)]
    pub fn sbaccess(&self) -> SBACCESS_R {
        SBACCESS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - When 1, every write to sbaddress0 automatically triggers a system bus read at the new address."]
    #[inline(always)]
    pub fn sbreadonaddr(&self) -> SBREADONADDR_R {
        SBREADONADDR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, indicates the system bus master is busy. (Whether the system bus itself is busy is related, but not the same thing.) This bit goes high immediately when a read or write is requested for any reason, and does not go low until the access is fully completed.  

 Writes to sbcs while sbbusy is high result in undefined behavior. A debugger must not write to sbcs until it reads sbbusy as 0."]
    #[inline(always)]
    pub fn sbbusy(&self) -> SBBUSY_R {
        SBBUSY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set when the debugger attempts to read data while a read is in progress, or when the debugger initiates a new access while one is already in progress (while sbbusy is set). It remains set until it’s explicitly cleared by the debugger.  

 While this field is set, no more system bus accesses can be initiated by the Debug Module."]
    #[inline(always)]
    pub fn sbbusyerror(&self) -> SBBUSYERROR_R {
        SBBUSYERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 1: The System Bus interface conforms to version 0.13.2 of the RISC-V debug spec."]
    #[inline(always)]
    pub fn sbversion(&self) -> SBVERSION_R {
        SBVERSION_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - When the Debug Module’s system bus master encounters an error, this field gets set. The bits in this field remain set until they are cleared by writing 1 to them. While this field is non-zero, no more system bus accesses can be initiated by the Debug Module.  

 An implementation may report “Other” (7) for any error condition. (Hazard3 does not use this value for any errors.)  

 0: There was no bus error.  

 1: There was a timeout.  

 2: A bad address was accessed.  

 3: There was an alignment error.  

 4: An access of unsupported size was requested.  

 7: Other.  

 Hazard3 raises an alignment error for any non-naturally-aligned bus transfer which would otherwise be a valid transfer."]
    #[inline(always)]
    #[must_use]
    pub fn sberror(&mut self) -> SBERROR_W<SBCS_SPEC> {
        SBERROR_W::new(self, 12)
    }
    #[doc = "Bit 15 - When 1, every read from sbdata0 automatically triggers a system bus read at the (possibly auto- incremented) address."]
    #[inline(always)]
    #[must_use]
    pub fn sbreadondata(&mut self) -> SBREADONDATA_W<SBCS_SPEC> {
        SBREADONDATA_W::new(self, 15)
    }
    #[doc = "Bit 16 - When 1, sbaddress is incremented by the access size (in bytes) selected in sbaccess after every system bus access."]
    #[inline(always)]
    #[must_use]
    pub fn sbautoincrement(&mut self) -> SBAUTOINCREMENT_W<SBCS_SPEC> {
        SBAUTOINCREMENT_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Select the access size to use for system bus accesses.  

 0: 8-bit  

 1: 16-bit  

 2: 32-bit  

 3: 64-bit  

 4: 128-bit  

 If sbaccess has an unsupported value when the DM starts a bus access, the access is not per formed and sberror is set to 4. (On Hazard3 the supported values are 8-bit, 16-bit and 32-bit.)"]
    #[inline(always)]
    #[must_use]
    pub fn sbaccess(&mut self) -> SBACCESS_W<SBCS_SPEC> {
        SBACCESS_W::new(self, 17)
    }
    #[doc = "Bit 20 - When 1, every write to sbaddress0 automatically triggers a system bus read at the new address."]
    #[inline(always)]
    #[must_use]
    pub fn sbreadonaddr(&mut self) -> SBREADONADDR_W<SBCS_SPEC> {
        SBREADONADDR_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set when the debugger attempts to read data while a read is in progress, or when the debugger initiates a new access while one is already in progress (while sbbusy is set). It remains set until it’s explicitly cleared by the debugger.  

 While this field is set, no more system bus accesses can be initiated by the Debug Module."]
    #[inline(always)]
    #[must_use]
    pub fn sbbusyerror(&mut self) -> SBBUSYERROR_W<SBCS_SPEC> {
        SBBUSYERROR_W::new(self, 22)
    }
}
#[doc = "System Bus Access Control and Status  

You can [`read`](crate::Reg::read) this register and get [`sbcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBCS_SPEC;
impl crate::RegisterSpec for SBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcs::R`](R) reader structure"]
impl crate::Readable for SBCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbcs::W`](W) writer structure"]
impl crate::Writable for SBCS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0040_7000;
}
#[doc = "`reset()` method sets SBCS to value 0x2000_0407"]
impl crate::Resettable for SBCS_SPEC {
    const RESET_VALUE: u32 = 0x2000_0407;
}
