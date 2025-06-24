

#[macro_export]
macro_rules! bitwise_mask {
    ($position: expr, $bits: expr) => {
        {
            let mut value = 0u64;
            let mut count = $bits;
            loop {
                count -= 1;
                value |= 1;
                if count <= 0 { break }
                value = value.shl(1);
            }
            value.shl($position)
        }
    };
}

#[macro_export]
macro_rules! bitwise_get {
    ($value: expr, $mask: expr, $shift: expr) => {
            (($value & $mask) as VoxelType).shr($shift)
    };
}
#[macro_export]
macro_rules! bitwise_set {
    ($value: expr, $mask: expr, $shift: expr, $new_value: expr) => {
        {
            $value = (($new_value as VoxelType).shl($shift) & *$mask) | ($value & !*$mask);
        }
    };
}

/// exports
pub(crate) use bitwise_set;
pub(crate) use bitwise_get;
pub(crate) use bitwise_mask;

