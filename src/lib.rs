
pub mod framework;
pub mod infotriple;
pub mod descnote;
pub mod infoedge;
pub mod types;

pub use framework::ig_tr::InfoTriple as InfoTriple;
pub use framework::ig_tr::InfoTable as InfoTable;
pub use framework::ig_desc::Descriptor as Descriptor;
pub use framework::ig_edge::model::info_edge::InfoEdge as InfoEdge;
pub use framework::ig_edge::model::info_graph::InfoGraph as InfoGraph;

