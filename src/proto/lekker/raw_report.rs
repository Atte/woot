#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum RawReportType {
    #[deprecated = "Removed from Wooting firmware"]
    RgbMainPart = 0,
    #[deprecated = "Removed from Wooting firmware"]
    DigitalProfileMainPart = 1,
    #[deprecated = "Removed from Wooting firmware"]
    AnalogProfileMainPart = 2,
    #[deprecated = "Removed from Wooting firmware"]
    AnalogProfileCurveChangeMapPart1 = 3,
    #[deprecated = "Removed from Wooting firmware"]
    AnalogProfileCurveChangeMapPart2 = 4,
    #[deprecated = "Removed from Wooting firmware"]
    MainMappingProfile = 5,
    #[deprecated = "Removed from Wooting firmware"]
    FunctionMappingProfile = 6,
    DeviceConfig = 7,
    SetDKSProfile = 8,
    RgbColorsPart = 9,
    #[deprecated = "Removed from Wooting firmware"]
    RgbEffects = 10,
    WootDevRawReport = 11,
    SerialNumber = 12,
    #[deprecated = "Removed from Wooting firmware"]
    RgbProfileBase = 13,
    RgbProfileColorsPart1 = 14,
    RgbProfileColorsPart2 = 15,
    #[deprecated = "Removed from Wooting firmware"]
    RgbProfileEffect = 16,
    KeyboardProfile = 17,
    GamepadMapping = 18,
    GamepadProfile = 19,
    MappingProfile = 20,
    ActuationProfile = 21,
    RgbProfileCore = 22,
    GlobalSettings = 23,
    AKCProfile = 24,
    RapidTriggerProfile = 25,
    ProfileMetadata = 26,
}
