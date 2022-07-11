use crate::{proto::Report, Result};
use bytes::{Buf, BufMut, Bytes};
use color_eyre::eyre::bail;
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
enum Type {
    Ping = 0,
    GetVersion = 1,
    ResetToBootloader = 2,
    GetSerial = 3,
    GetRgbProfileCount = 4,
    #[deprecated = "Removed from Wooting firmware"]
    GetCurrentRgbProfileIndex = 5,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbMainProfile = 6,
    ReloadProfile0 = 7,
    SaveRgbProfile = 8,
    GetDigitalProfilesCount = 9,
    GetAnalogProfilesCount = 10,
    GetCurrentKeyboardProfileIndex = 11,
    GetDigitalProfile = 12,
    GetAnalogProfileMainPart = 13,
    GetAnalogProfileCurveChangeMapPart1 = 14,
    GetAnalogProfileCurveChangeMapPart2 = 15,
    GetNumberOfKeys = 16,
    GetMainMappingProfile = 17,
    GetFunctionMappingProfile = 18,
    GetDeviceConfig = 19,
    GetAnalogValues = 20,
    KeysOff = 21,
    KeysOn = 22,
    ActivateProfile = 23,
    GetDKSProfile = 24,
    DoSoftReset = 25,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbColorsPart1 = 26,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbColorsPart2 = 27,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbEffects = 28,
    RefreshRgbColors = 29,
    WootDevSingleColor = 30,
    WootDevResetColor = 31,
    WootDevResetAll = 32,
    WootDevInit = 33,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbProfileBase = 34,
    GetRgbProfileColorsPart1 = 35,
    GetRgbProfileColorsPart2 = 36,
    #[deprecated = "Removed from Wooting firmware"]
    GetRgbProfileEffect = 37,
    ReloadProfile = 38,
    GetKeyboardProfile = 39,
    GetGamepadMapping = 40,
    GetGamepadProfile = 41,
    SaveKeyboardProfile = 42,
    ResetFlash = 43,
    SetRawScanning = 44,
    StartXinputDetection = 45,
    StopXinputDetection = 46,
    SaveDKSProfile = 47,
    GetMappingProfile = 48,
    GetActuationProfile = 49,
    GetRgbProfileCore = 50,
    GetGlobalSettings = 51,
    GetAKCProfile = 52,
    SaveAKCProfile = 53,
    GetRapidTriggerProfile = 54,
    GetProfileMetadata = 55,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ping;
impl Report for Ping {
    const TYPE: u8 = Type::Ping as u8;
    type Response = ();

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        if buffer.get_u8() != 255 {
            bail!("invalid ping response");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetVersion;
impl Report for GetVersion {
    const TYPE: u8 = Type::GetVersion as u8;
    type Response = (u8, u8, u8);

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        Ok((buffer.get_u8(), buffer.get_u8(), buffer.get_u8()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Serial {
    pub supplier_number: u16,
    pub year: u8,
    pub week_number: u8,
    pub product_number: u16,
    pub revision_number: u16,
    pub product_id: u16,
    pub production_stage: Option<ProductionStage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum ProductionStage {
    Mass = 0,
    PVT = 1,
    DVT = 2,
    EVT = 3,
    Prototype = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetSerial;
impl Report for GetSerial {
    const TYPE: u8 = Type::GetSerial as u8;
    type Response = Serial;

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        Ok(Serial {
            supplier_number: buffer.get_u16_le(),
            year: buffer.get_u8(),
            week_number: buffer.get_u8(),
            product_number: buffer.get_u16_le(),
            revision_number: buffer.get_u16_le(),
            product_id: buffer.get_u16_le(),
            production_stage: ProductionStage::try_from_primitive(buffer.get_u8()).ok(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActivateProfile(pub u8);
impl Report for ActivateProfile {
    const TYPE: u8 = Type::ActivateProfile as u8;
    type Response = ();

    #[inline]
    fn write_request(&self, buffer: &mut impl BufMut) {
        buffer.put_u8(self.0);
    }

    #[inline]
    fn read_response(_buffer: impl Buf) -> Result<Self::Response> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetRgbProfileCore(pub u8);
impl Report for GetRgbProfileCore {
    const TYPE: u8 = Type::GetRgbProfileCore as u8;
    type Response = Bytes;

    #[inline]
    fn write_request(&self, buffer: &mut impl BufMut) {
        buffer.put_u8(self.0);
    }

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        Ok(buffer.copy_to_bytes(buffer.remaining()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetRgbProfileColorsPart1(pub u8);
impl Report for GetRgbProfileColorsPart1 {
    const TYPE: u8 = Type::GetRgbProfileColorsPart1 as u8;
    type Response = Bytes;

    #[inline]
    fn write_request(&self, buffer: &mut impl BufMut) {
        buffer.put_u8(self.0);
    }

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        Ok(buffer.copy_to_bytes(buffer.remaining()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GetRgbProfileColorsPart2(pub u8);
impl Report for GetRgbProfileColorsPart2 {
    const TYPE: u8 = Type::GetRgbProfileColorsPart2 as u8;
    type Response = Bytes;

    #[inline]
    fn write_request(&self, buffer: &mut impl BufMut) {
        buffer.put_u8(self.0);
    }

    fn read_response(mut buffer: impl Buf) -> Result<Self::Response> {
        Ok(buffer.copy_to_bytes(buffer.remaining()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReloadProfile;
impl Report for ReloadProfile {
    const TYPE: u8 = Type::ReloadProfile as u8;
    type Response = ();

    #[inline]
    fn read_response(_buffer: impl Buf) -> Result<Self::Response> {
        Ok(())
    }
}
