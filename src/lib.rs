#[repr(C)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct HashAndIndex {
    pub hash: i32,
    pub index: i32,
}

/// Computes a hash value from a 3D grid position using large prime multipliers.
///
/// # Safety
/// - `grid_pos` must be a valid, non-null pointer to an `Int3`.
/// - The pointer must be properly aligned and point to initialized memory.
#[no_mangle]
pub unsafe extern "C" fn hash(grid_pos: *const Int3) -> i32 {
    let pos = unsafe { &*grid_pos };

    let x = pos.x.wrapping_mul(73856093);
    let y = pos.y.wrapping_mul(19349663);
    let z = pos.z.wrapping_mul(83492791);
    x ^ y ^ z
}

#[no_mangle]
pub extern "C" fn grid_position(position: Float3, cell_size: f32) -> Int3 {
    Int3 {
        x: (position.x / cell_size).floor() as i32,
        y: (position.y / cell_size).floor() as i32,
        z: (position.z / cell_size).floor() as i32,
    }
}

/// Performs a binary search on a sorted array of `HashAndIndex` using the hash of the provided grid position.
/// Writes the computed hash to `out_hash`.
///
/// # Safety
/// - `grid_pos` must be a valid, non-null pointer to an `Int3`.
/// - `array` must point to a valid contiguous buffer of at least `length` elements.
/// - `out_hash` must be a valid, non-null pointer to an `i32`.
/// - All pointers must be properly aligned and not aliased for mutable access.
#[no_mangle]
pub unsafe extern "C" fn binary_search_first_grid(
    grid_pos: *const Int3,
    array: *const HashAndIndex,
    length: i32,
    out_hash: *mut i32,
) -> i32 {
    let hash = hash(grid_pos);

    unsafe { *out_hash = hash };

    let array = unsafe { std::slice::from_raw_parts(array, length as usize) };

    let mut left = 0usize;
    let mut right = length as usize;

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_hash = unsafe { array.get_unchecked(mid).hash };

        if mid_hash < hash {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < array.len() && unsafe { array.get_unchecked(left).hash } == hash {
        left as i32
    } else {
        -1
    }
}
