#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
pub enum InstructionType {
  // Swaps
  RaydiumAmmV4Swap,
  RaydiumCpmmSwap,
  RaydiumLaunchpadSwap,
  PumpswapSwap,
  PfBondingCurveSwap,
  // Bubblemapping
  Link,

  // TODO Swaps to add
  MeteoraDammV2Swap,
  MeteoraDbcSwap,
  MeteoraAmmSwap,


  // TODO Add/remove liquidity AMMs
  RaydiumAmmV4AddLiquidity,
  RaydiumAmmV4RemoveLiquidity,
  RaydiumCpmmAddLiquidity,
  RaydiumCpmmRemoveLiquidity,
  PumpswapAddLiquidity,
  PumpswapRemoveLiquidity,

  // TODO Add CLMM support
  // TODO add Jupiter Perps support
  // TODO add private AMMs support

  PfBondingCurveCreate,

  // None of the above
  None,

  
}
