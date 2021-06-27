pub mod basic_table_arrows;
pub mod battery_display;
pub mod cpu_basic;
pub mod cpu_graph;
pub mod disk_table;
pub mod mem_basic;
pub mod mem_graph;
pub mod network_basic;
pub mod network_graph;
pub mod process_table;
pub mod temp_table;

pub use basic_table_arrows::BasicTableArrows;
pub use battery_display::BatteryDisplayWidget;
pub use cpu_basic::CpuBasicWidget;
pub use cpu_graph::CpuGraphWidget;
pub use disk_table::DiskTableWidget;
pub use mem_basic::MemBasicWidget;
pub use mem_graph::MemGraphWidget;
pub use network_basic::NetworkBasicWidget;
pub use network_graph::NetworkGraphWidget;
pub use process_table::ProcessTableWidget;
pub use temp_table::TempTableWidget;
