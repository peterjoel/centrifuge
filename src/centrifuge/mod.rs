pub mod perm;
pub mod transient;


pub trait Message {
    fn get_sequence(&self) -> usize;
    fn get_data(&self) -> &[u8];
}

#[derive(Debug)]
pub struct RawMsg<'a> {
    pub data: &'a [u8],
}

pub trait Store<'a> {
    type Msg: Message;

    fn write<W>(&mut self, writer: W) -> Self::Msg
        where W: FnOnce(&mut [u8]) -> usize;

    fn write_msg(&mut self, raw: RawMsg) -> Self::Msg {
        let num_bytes = raw.data.len();
        self.write(|buf| {
            buf[0 .. num_bytes].clone_from_slice(&raw.data);
            num_bytes
        })
    }
}


pub struct Centrifuge<'a, T: 'a, P> {
    store: &'a mut T,
    processor: P,
}

impl <'a, T, P> Centrifuge<'a, T, P> 
    where T: Store<'a>,
          P: FnMut(T::Msg)
{ 
    pub fn new(store: &'a mut T, processor: P) -> Centrifuge<'a, T, P>  {
        Centrifuge { 
            store: store,
            processor: processor,
        }
    }

    pub fn receive_msg(&mut self, data: &[u8]) {
        let raw = RawMsg { data: data };
        let msg = self.store.write_msg(raw);
        // TODO handle panics. We should be able to pinpoint the exact message that caused a panics
        // and then take appropriate action and possibly recover.
        (self.processor)(msg);
    }
}

