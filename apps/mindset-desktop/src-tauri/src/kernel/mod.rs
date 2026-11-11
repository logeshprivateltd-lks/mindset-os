//! MindSet OS Kernel Subsystem
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

pub mod event_bus;
pub mod lifecycle;
pub mod registry;
pub mod scheduler;

#[allow(unused_imports)]
pub use event_bus::{EventBus, HardwareCapabilities, KernelEvent};
#[allow(unused_imports)]
pub use lifecycle::{BootDiagnostics, BootPhase, KernelBootSequencer};
#[allow(unused_imports)]
pub use registry::{KernelError, KernelService, ServiceHealth, ServiceRegistry, ServiceStatus};
pub use scheduler::KernelScheduler;
