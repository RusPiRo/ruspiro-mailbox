# Changelog

## :melon: v0.4.1

This maintenance version moves the pipeline build to github actions and fixes the versions of the dependent crates.

- ### :wrench: Maintenance

  - migrate pipeline to github actions
  - fix the dependent crate versions to reduce the probability of regressions if those versions changes
  
## :peach: v0.4.0

This version introduces the new pipeline build configuration for travis-ci for crate verification and publishing. The API has been slightly adjuasted and the singleton usage for the mailbox structure has been removed to allow the user of this crate how to handle the mailbox instance and ensure that it is adhering to the rule to exist only once.

- ### :wrench: Maintenance

  - introduce the new pipeline configuration

- ### :bulb: Features

  - Introduce the usage of the property tag `VchiqInit`

## :banana: v0.3.1

- ### :wrench: Maintenance
  
  - use `cargo make` to stabilize build

## :carrot: v0.3.0
  
- ### :bulb: Features
  
  Support several Raspberry Pi mailbox property tags like:

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

  Support batch processing of mailbox property tags. This is a prerequisit for framebuffer related property tags as they need to be processed at once by the VC/GPU
