module sddia_anchor::anchor {
    use iota::object::{Self, UID};
    use iota::transfer;
    use iota::tx_context::{Self, TxContext};
    use std::string;

    public struct ImmutableAnchor has key, store {
        id: UID,
        payload: string::String,
    }

    public entry fun publish_immutable(payload: vector<u8>, ctx: &mut TxContext) {
        let anchor = ImmutableAnchor {
            id: object::new(ctx),
            payload: string::utf8(payload),
        };
        transfer::public_transfer(anchor, tx_context::sender(ctx));
    }
}
