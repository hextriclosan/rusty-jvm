//! Policy-v1 managed storage accounting. This is not an RSS measurement.
use std::fmt;

pub(crate) const CHARGE_POLICY_VERSION: u32 = 1;
const _: () = assert!(CHARGE_POLICY_VERSION == 1);

/// Small, inline failures: creating or propagating one never allocates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocError {
    Capacity,
    Arithmetic,
    IdentityExhausted,
    HostAllocation,
    NegativeLength(i32),
    InvalidLayout,
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Capacity => "managed heap capacity exhausted",
            Self::Arithmetic => "managed heap accounting overflow",
            Self::IdentityExhausted => "heap identity space exhausted",
            Self::HostAllocation => "host storage reservation failed",
            Self::NegativeLength(_) => "negative array length",
            Self::InvalidLayout => "invalid allocation layout",
        })
    }
}

pub(crate) type AllocResult<T> = Result<T, AllocError>;

#[derive(Debug)]
pub(crate) struct Budget {
    pub(crate) used: u64,
    pub(crate) reserved: u64,
    pub(crate) max: u64,
    pub(crate) enforced: bool,
}

impl Budget {
    pub(crate) const fn new(max: u64, enforced: bool) -> Self {
        Self {
            used: 0,
            reserved: 0,
            max,
            enforced,
        }
    }

    pub(crate) fn reserve(&mut self, charge: u64) -> AllocResult<()> {
        let reserved = self
            .reserved
            .checked_add(charge)
            .ok_or(AllocError::Arithmetic)?;
        let total = self
            .used
            .checked_add(reserved)
            .ok_or(AllocError::Arithmetic)?;
        if self.enforced && total > self.max {
            return Err(AllocError::Capacity);
        }
        self.reserved = reserved;
        Ok(())
    }

    pub(crate) fn commit(&mut self, reservation: u64, recorded: u64) -> AllocResult<()> {
        if recorded > reservation {
            return Err(AllocError::InvalidLayout);
        }
        let reserved = self
            .reserved
            .checked_sub(reservation)
            .ok_or(AllocError::Arithmetic)?;
        let used = self
            .used
            .checked_add(recorded)
            .ok_or(AllocError::Arithmetic)?;
        let total = used.checked_add(reserved).ok_or(AllocError::Arithmetic)?;
        if self.enforced && total > self.max {
            return Err(AllocError::Capacity);
        }
        self.reserved = reserved;
        self.used = used;
        Ok(())
    }

    pub(crate) fn cancel(&mut self, charge: u64) -> AllocResult<()> {
        self.reserved = self
            .reserved
            .checked_sub(charge)
            .ok_or(AllocError::Arithmetic)?;
        Ok(())
    }

    pub(crate) fn reclaim(&mut self, recorded: u64) -> AllocResult<()> {
        self.used = self
            .used
            .checked_sub(recorded)
            .ok_or(AllocError::Arithmetic)?;
        Ok(())
    }
}

pub(crate) fn bytes(count: usize, width: usize) -> AllocResult<u64> {
    let bytes = count.checked_mul(width).ok_or(AllocError::Arithmetic)?;
    u64::try_from(bytes).map_err(|_| AllocError::Arithmetic)
}

pub(crate) fn aligned(bytes: u64) -> AllocResult<u64> {
    bytes
        .checked_add(7)
        .map(|value| value & !7)
        .ok_or(AllocError::Arithmetic)
}

pub(crate) fn add(total: &mut u64, bytes: u64) -> AllocResult<()> {
    *total = total
        .checked_add(aligned(bytes)?)
        .ok_or(AllocError::Arithmetic)?;
    Ok(())
}

pub(crate) fn copy_string(value: &str) -> AllocResult<String> {
    let mut result = String::new();
    result
        .try_reserve_exact(value.len())
        .map_err(|_| AllocError::HostAllocation)?;
    result.push_str(value);
    Ok(result)
}

pub(crate) fn copy_slice<T: Copy>(value: &[T]) -> AllocResult<Vec<T>> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(value.len())
        .map_err(|_| AllocError::HostAllocation)?;
    result.extend_from_slice(value);
    Ok(result)
}
