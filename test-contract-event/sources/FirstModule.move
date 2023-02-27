module CustomTutorial::CustomContract {
    use aptos_std::event;
    use std::signer;
    use aptos_framework::account::new_event_handle;

    struct MessageHolder has key {
        message_change_events: event::EventHandle<CustomEvent>,
    }

    struct CustomEvent has drop, store {
        internal_data: u8
    }

    entry fun init_module(creator: &signer) {
        move_to(
            creator,
            MessageHolder { message_change_events: new_event_handle<CustomEvent>(creator) }
        );
    }

    public entry fun custom_call(account: signer) acquires MessageHolder {
        let account_addr = signer::address_of(&account);
        if (!exists<MessageHolder>(account_addr)) {
            move_to(&account, MessageHolder {
                message_change_events: new_event_handle<CustomEvent>(&account),
            })
        } else {
            let old_message_holder = borrow_global_mut<MessageHolder>(account_addr);

            event::emit_event(&mut old_message_holder.message_change_events, CustomEvent {
                internal_data: 5
            });
        }
    }
}
