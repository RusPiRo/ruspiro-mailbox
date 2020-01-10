# Changelog
## :carrot: v0.3.0
  - ### :bulb: Features
    Support several Raspberry Pi mailbox property tags like:<br>
    - FirmwareRevisionGet
    - BoardModelGet
    - BoardRevisionGet
    - BoardSerialGet
    - ArmMemoryGet
    - BoardMACAddressGet
    - VcMemoryGet
    - DmaChannelsGet
    - PowerStateGet
    - PowerStateSet
    - ClockStateGet
    - ClockStateSet
    - ClockrateGet
    - ClockrateSet
    - MaxClockrateGet
    - MinClockrateGet
    - VoltageGet
    - VoltageSet
    - MaxVoltageGet
    - MinVoltageGet
    - TemperatureGet
    - MaxTemperatureGet
    - FramebufferAllocate
    - FramebufferRelease
    - BlankScreen
    - PhysicalSizeGet
    - PhysicalSizeSet
    - VirtualSizeGet
    - VirtualSizeSet
    - DepthGet
    - DepthSet
    - PixelOrderGet
    - PixelOrderSet
    - AlphaModeGet
    - AlphaModeSet
    - PitchGet
    - VirtualOffsetGet
    - VirtualOffsetSet
    - OverscanGet
    - OverscanSet
    - PaletteGet
    - PaletteSet

  Support batch processing of mailbox property tags. This is a prerequisit for framebuffer related 
  property tags as they need to be processed at once by the VC/GPU<br>
