pub mod centrifuge;

use std::str;

#[test]
pub fn test(){
    use centrifuge::*;

    struct MyProcessor {
        sequence_nums: Vec<usize>
    }

    let mut sequence_nums = Vec::<usize>::new();

    impl Processor for MyProcessor {
        fn process(&mut self, msg: Msg) {
            self.sequence_nums.push(msg.sequence);
        }
    }
    
    let mut log = vec![0; 12_000];

    let mut processor = MyProcessor { sequence_nums: Vec::new() };
    {
        let mut history = DefaultHistory::new(&mut log);

        let mut centrifuge = Centrifuge::new(&mut history, &mut processor);

        let data = b"hello";
        centrifuge.receive_msg( &data[..] );

        let data = b"bye";
        centrifuge.receive_msg( &data[..] );
    }


    assert_eq!(&vec![1, 2], &processor.sequence_nums);
    assert_eq!(&b"hellobye"[..], &log[0..8]);
}