pub enum PstreamFormat {
    AnyFormat = 0,
    AsciiFormat = 1,
    BinaryFormat = 2,
    XdrFormat = 3,
    AsciihexFormat = 4,
}

mod load;
mod save;

pub use load::Load;
pub use save::Save;
