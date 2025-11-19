use dep::std;

global MERKLE_TREE_DEPTH: u32 = 20;

fn main(user_secret: Field,
    vote_choice: Field,
    merkle_path: [Field; MERKLE_TREE_DEPTH],
    merkle_indices: [u1; MERKLE_TREE_DEPTH],
    merkle_root: pub Field,
    nullifier: pub Field,
    poll_id: pub Field,
    max_options: pub Field
) {
    assert((vote_choice as u32) < (max_options as u32));
    assert((vote_choice as u32) >= 0);

    // verify that the nullifier is correctly computed
    let computed_nullifier = std::hash::pedersen_hash([user_secret, poll_id]);
    // std::println(f"Computed nullifier: {computed_nullifier}");
    assert(nullifier == computed_nullifier);

    // Verify that the user is in the eligible voters merkle tree
    let leaf = std::hash::pedersen_hash([user_secret]);
    // std::println(f"Leaf hash: {leaf}");
    let is_valid_member = verify_merkle_membership(
        leaf,
        merkle_root,
        merkle_path,
        merkle_indices
    );
    assert(is_valid_member);

    // ensure user_secret is not 0
    assert(user_secret != 0);

    std::println(f"Vote verified for poll {poll_id}");
}

fn verify_merkle_membership(
    leaf: Field,
    root: Field,
    path: [Field; MERKLE_TREE_DEPTH],
    indices: [u1; MERKLE_TREE_DEPTH]
) -> bool {
    let mut current = leaf;

    // std::println(f"Starting with leaf: {leaf}");
    for i in 0..MERKLE_TREE_DEPTH {
        let path_element = path[i];
        // std::println(f"Step {i}: current = {current}, path_element = {path_element}");
        let is_right = indices[i];

        if is_right == 0 {
            // Current is left child
            current = std::hash::pedersen_hash([current, path_element]);
        } else {
            // Current is right child
            current = std::hash::pedersen_hash([path_element, current]);
        }
    }
    // std::println(f"Final computed root: {current}");
    current == root
}
