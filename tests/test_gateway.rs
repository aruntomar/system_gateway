use system_gateway::gateway;

#[test]
fn test_gateway() {
    let length = gateway().unwrap().split('.').count();
    // assert_eq!(gateway(), "192.168.0.1");
    assert!(!gateway().unwrap().is_empty());
    assert!(length == 4);
}

// if command not found then it should return an error.
// #[test]
// fn test_gw_fails() {
//     assert!(gateway().is_err());
// }