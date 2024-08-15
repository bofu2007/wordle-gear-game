use gstd::ActorId;
use gtest::{Log, Program, System};
use wordle_game_io::*;

const USER: u64 = 3;
const TARGET_PROGRAM_ADDRESS: u64 = 2;

#[test]
fn success_test() {
    // Create a new testing environment.
    let system = System::new();
    system.init_logger();
    // Get proxy program of the root crate with provided system.
    let proxy_program = Program::current(&system);
    // Get target program
    let target_program = Program::from_file(
        &system,
        "../wordle/target/wasm32-unknown-unknown/debug/wordle_game.wasm",
    );
    // The target program is initialized with an empty payload message
    let result = target_program.send_bytes(USER, []);
    assert!(!result.main_failed());

    let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
    // The proxy program is initialized using target_program in the payload message
    let res = proxy_program.send(USER, target_program_address);
    assert!(!res.main_failed());

    // Send with the message we want to receive back
    let result = proxy_program.send(
        USER,
        Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 }),
    );
    assert!(!result.main_failed());

    let log = Log::builder().source(1).dest(3).payload(Event::MessageSent);

    assert!(result.contains(&log));

    // user attempts to send another message to a proxy program while it is still processing the first message. It is expected that the proxy program will reply with the event `MessageAlreadySent`.
    let result = proxy_program.send(
        USER,
        Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 }),
    );
    assert!(!result.main_failed());

    let log = Log::builder().source(1).dest(3).payload(Event::WrongStatus);
    assert!(result.contains(&log));

    let _result = system.spend_blocks(3);
    // if target_program handle() exist `exec::wait();`, `Event::NoReplyReceived` will be received.
    // Event::NoReplyReceived 1
    // let log = Log::builder()
    //     .source(1)
    //     .dest(3)
    //     .payload(Event::NoReplyReceived);
    // assert!(result[0].contains(&log));

    // Event::NoReplyReceived 2
    // check that the proxy message has arrived,
    // which means that the message was successfully sent to the target program
    // let mailbox = system.get_mailbox(USER);
    // let log = Log::builder()
    //     .source(1)
    //     .dest(3)
    //     .payload(Event::NoReplyReceived);
    // assert!(mailbox.contains(&log));
}
