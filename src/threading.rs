pub enum EmulatorCommand {
    Go,
    Step(u32),
    GetMemory,
    GetRegisters,
    Pause,
}

pub enum EmulatorResponse {
    FrameBuffer(Vec<[u8;4]>)
}

impl EmulatorResponse {
    pub fn match_response(&self) {
        match self {

        }
    }
}

pub trait ThreadedEmulator {

    /// Creates a new instance of the associated type Computer. 
    /// 
    /// The Sender should be hooked up into wherever graphics updates will come from 
    /// (e.g. frame buffer updates) 
    fn new(sender_from_computer: Sender<EmulatorResponse>) -> Self;

    /// Performs the action corresponding to the received EmulatorCommand
    fn match_received_command(&self, command: EmulatorCommand) -> std::result::Result<(), String>;

    ///
    fn initialize() -> (Sender<EmulatorCommand>, Receiver<EmulatorResponse>)
    where Self: Sized {
        let (sender_to_computer, receiver_to_computer) = channel::<EmulatorCommand>();
        let (sender_from_computer, receiver_from_computer) = channel::<EmulatorResponse>();

        std::thread::spawn(move || {
            let computer = Self::new(sender_from_computer);

            loop {
                if let Ok(command) = receiver_to_computer.try_recv() {
                    if let Err(e) = computer.match_received_command(command){
                        eprintln!("{e}");
                    };
                }
            }
        });
       
       (sender_to_computer, receiver_from_computer)
    }
}