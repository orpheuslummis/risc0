use zktables::execute_and_prove;
use zktables_core::Votes;
use zktables_methods::ZKTABLES_GUEST_ID;

#[test]
fn proper() {
    let votes: Votes = vec![1, 2, 3];
    let (receipt, output) = execute_and_prove(votes);
    let v = receipt.verify(ZKTABLES_GUEST_ID.into());
    assert!(v.is_ok());
    assert_eq!(output.result, 2);
}
