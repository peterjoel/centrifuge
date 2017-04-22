pub mod centrifuge;





#[cfg(test)]
pub mod test {
    use centrifuge::*;

    #[test]
    pub fn test_perm() {
        use centrifuge::perm::*;
        
        let mut sequence_nums = Vec::<usize>::new();
        let mut log = vec![0; 12_000];

        {
            let mut store = PermStore::new(&mut log);

            let mut centrifuge = Centrifuge::new(&mut store, |msg| sequence_nums.push(msg.get_sequence()));

            let data = b"hello";
            centrifuge.receive_msg( &data[..] );

            let data = b"bye";
            centrifuge.receive_msg( &data[..] );
        }


        assert_eq!(&vec![1, 2], &sequence_nums);
        assert_eq!(&b"hellobye"[..], &log[0..8]);
    }
}