//! 1.17 (755): отличия провода от 1.17.1.
#![allow(clippy::unwrap_used)]

use ezzmc_net::v1201::from_1_17;
use ezzmc_protocol::wire::Writer;

#[test]
fn containers_get_state_id_and_single_remove_becomes_list() {
    let mut w = Writer::new();
    w.u8(0).i16(2).bytes(&[0, 0]);
    let (name, b) = from_1_17("container_set_content", &w.into_vec())
        .unwrap()
        .unwrap();
    assert_eq!(name, "container_set_content");
    // window, stateId, count, 2 пустых слота, пустой курсор
    assert_eq!(b, [0, 0, 2, 0, 0, 0]);

    let (_, b) = from_1_17("container_set_slot", &[0, 0, 36, 0])
        .unwrap()
        .unwrap();
    assert_eq!(b, [0, 0, 0, 36, 0]);

    let (name, b) = from_1_17("remove_entity", &[7]).unwrap().unwrap();
    assert_eq!((name, b), ("remove_entities", vec![1, 7]));

    assert!(from_1_17("keep_alive", &[0; 8]).unwrap().is_none());
    assert!(from_1_17("container_set_content", &[0]).is_err());
}
