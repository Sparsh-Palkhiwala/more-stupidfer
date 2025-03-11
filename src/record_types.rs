#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum RecordType {
    FAR,
    ATR,
    MIR,
    MRR,
    PCR,
    HBR,
    SBR,
    PMR,
    PGR,
    PLR,
    RDR,
    SDR,
    WIR,
    WRR,
    WCR,
    PIR,
    PRR,
    TSR,
    PTR,
    MPR,
    FTR,
    BPS,
    EPS,
    GDR,
    DTR,
    PSR,
    VUR,
    InvalidRecord,
}

impl RecordType {
    pub fn new(rec_typ: u8, rec_sub: u8) -> Self {
        match rec_typ {
            0 => match rec_sub {
                10 => Self::FAR,
                20 => Self::ATR,
                30 => Self::VUR,
                _ => Self::InvalidRecord,
            },
            1 => match rec_sub {
                10 => Self::MIR,
                20 => Self::MRR,
                30 => Self::PCR,
                40 => Self::HBR,
                50 => Self::SBR,
                60 => Self::PMR,
                62 => Self::PGR,
                63 => Self::PLR,
                70 => Self::RDR,
                80 => Self::SDR,
                90 => Self::PSR,
                _ => Self::InvalidRecord,
            },
            2 => match rec_sub {
                10 => Self::WIR,
                20 => Self::WRR,
                30 => Self::WCR,
                _ => Self::InvalidRecord,
            },
            5 => match rec_sub {
                10 => Self::PIR,
                20 => Self::PRR,
                _ => Self::InvalidRecord,
            },
            10 => match rec_sub {
                30 => Self::TSR,
                _ => Self::InvalidRecord,
            },
            15 => match rec_sub {
                10 => Self::PTR,
                15 => Self::MPR,
                20 => Self::FTR,
                _ => Self::InvalidRecord,
            },
            20 => match rec_sub {
                10 => Self::BPS,
                20 => Self::EPS,
                _ => Self::InvalidRecord,
            },
            50 => match rec_sub {
                10 => Self::GDR,
                30 => Self::DTR,
                _ => Self::InvalidRecord,
            },
            _ => Self::InvalidRecord,
        }
    }
}
