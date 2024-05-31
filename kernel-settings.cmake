# Adapted from seL4/configs/X64_verified.cmake

set(KernelPlatform "pc99" CACHE STRING "")
set(KernelSel4Arch "x86_64" CACHE STRING "")
set(KernelVerificationBuild OFF CACHE BOOL "") # Verification has to be off for kernel printing to work
set(KernelMaxNumNodes "1" CACHE STRING "")
set(KernelOptimisation "-O2" CACHE STRING "")
set(KernelRetypeFanOutLimit "256" CACHE STRING "")
set(KernelBenchmarks "none" CACHE STRING "")
set(KernelDangerousCodeInjection OFF CACHE BOOL "")
set(KernelFastpath ON CACHE BOOL "")
set(KernelPrinting ON CACHE BOOL "")
set(KernelNumDomains 16 CACHE STRING "")
set(KernelRootCNodeSizeBits 19 CACHE STRING "")
set(KernelFSGSBase "inst" CACHE STRING "")

set(KernelSupportPCID OFF CACHE BOOL "")
set(KernelFPU FXSAVE CACHE STRING "")
set(KernelArch x86 CACHE STRING "")
