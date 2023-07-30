pub mod columnmap;

const DEFAULT_DELIMITER: char = ',';

pub enum TimestampFormat {
    Rfc3339,
    Rfc3339Nano,
    Number,
}

pub enum DataType {
    Measurement,
    Tag,
    DateTime(TimestampFormat),
    Field,
    Ignored,
    String,
    Double,
    Long,
    UnsignedLong,
    Boolean,
}
