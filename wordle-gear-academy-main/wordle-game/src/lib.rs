#![no_std]
use gstd::{exec, msg, Decode, Encode, TypeInfo};

static mut SEED: u8 = 0;

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    Hello,
    HowAreYou,
    MakeRandomNumber { range: u8 },
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    Hello,
    Fine,
    Number(u8),
}

#[no_mangle]
extern "C" fn handle() {
    // exec::wait();
    let action: Action = msg::load().expect("Error in decode message");
    let reply = match action {
        Action::Hello => Event::Hello,
        Action::HowAreYou => Event::Fine,
        Action::MakeRandomNumber { range } => {
            let seed = unsafe { SEED };
            unsafe { SEED = SEED.wrapping_add(1) };
            let mut random_input: [u8; 32] = exec::program_id().into();
            random_input[0] = random_input[0].wrapping_add(seed);
            let (random, _) = exec::random(random_input).expect("Error in getting random number");
            Event::Number(random[0] % range)
        }
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}
