
pub mod perm;


pub trait Message<'a> {
    fn get_sequence(&self) -> usize;
    fn get_data(&self) -> &'a [u8];
}

pub struct RawMsg<'a> {
    pub data: &'a [u8],
}

pub trait Store<'a> {
    type Msg: Message<'a>;
    fn append(&mut self, raw: RawMsg) -> Self::Msg;
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
        let msg = self.store.append(raw);
        // TODO handle panics. We should be able to pinpoint the exact message that caused a panics
        // and then take appropriate action and possibly recover.
        (self.processor)(msg);
    }
}

