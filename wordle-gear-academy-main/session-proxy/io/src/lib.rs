#![no_std]
use gmeta::{InOut, Metadata};
use gstd::prelude::*;

// the metadata to be used by the [IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network) portal.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct WordleMetadata;

impl Metadata for WordleMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MessageAction {
    // arbitrary actions should be supported in the dApp (defined by dApp author)
    Hello,
    HowAreYou,
    MakeRandomNumber { range: u8 },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    SendMessage(MessageAction), // action to send a message to the target program
    CheckReply,                 // action to check for a response
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    // arbitrary replies to the action
    Hello,
    Fine,
    Number(u8),
    MessageSent,        // event confirming successful message sent from Proxy to Target
    MessageAlreadySent, // event confirming successful message sent from Proxy to Target
    WrongStatus,
    NoReplyReceived,
}
