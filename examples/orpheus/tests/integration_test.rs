use orpheus::execute_and_prove;
use orpheus_core::Votes;
use orpheus_methods::ORPHEUS_GUEST_ID;

#[test]
fn proper() {
    let votes: Votes = vec![1, 2, 3];
    let (receipt, output) = execute_and_prove(votes);
    let v = receipt.verify(ORPHEUS_GUEST_ID.into());
    assert!(v.is_ok());
    assert!(output.result == 3);
}
