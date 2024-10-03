use std::thread::spawn;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use without_channels::data::TicketDraft;
use without_channels::store::TicketStore;

#[test]
fn works() {
    let store = TicketStore::new();

    let mut store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store1.add_ticket(draft)
    });

    let mut store2 = store.clone();
    let client2 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store2.add_ticket(draft)
    });

    let ticket_id1 = client1.join().unwrap();
    let ticket_id2 = client2.join().unwrap();

    let reader = store;

    let ticket1 = reader.get(ticket_id1);
    assert!(ticket1.is_none());

    let ticket2 = reader.get(ticket_id2);
    assert!(ticket2.is_none());
}
