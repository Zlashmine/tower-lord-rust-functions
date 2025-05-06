use rust_unity_bridge::{binary_search_first_grid, hash, HashAndIndex, Int3};

#[test]
fn test_hash() {
    let pos = Int3 { x: 1, y: 2, z: 3 };
    let expected = {
        let x = pos.x.wrapping_mul(73856093);
        let y = pos.y.wrapping_mul(19349663);
        let z = pos.z.wrapping_mul(83492791);
        x ^ y ^ z
    };
    let result = hash(&pos);
    assert_eq!(result, expected);
}

#[test]
fn test_binary_search_first_grid_found() {
    let grid_pos = Int3 { x: 1, y: 2, z: 3 };
    let expected_hash = hash(&grid_pos);

    let array = [
        HashAndIndex {
            hash: expected_hash - 10,
            index: 0,
        },
        HashAndIndex {
            hash: expected_hash,
            index: 1,
        },
        HashAndIndex {
            hash: expected_hash + 10,
            index: 2,
        },
    ];

    let mut out_hash = -1;
    let index =
        binary_search_first_grid(&grid_pos, array.as_ptr(), array.len() as i32, &mut out_hash);

    assert_eq!(out_hash, expected_hash);
    assert_eq!(index, 1);
}

#[test]
fn test_binary_search_first_grid_not_found() {
    let array = [
        HashAndIndex { hash: 10, index: 0 },
        HashAndIndex { hash: 20, index: 1 },
        HashAndIndex { hash: 30, index: 2 },
    ];

    let pos = Int3 { x: 1, y: 1, z: 1 };
    let mut out_hash = -1;
    let result = binary_search_first_grid(&pos, array.as_ptr(), array.len() as i32, &mut out_hash);

    assert_eq!(result, -1);
    assert_eq!(out_hash, hash(&pos));
}
