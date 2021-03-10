mod test {
    use cmp_polymorphism::enum_obj;
    use cmp_polymorphism::trait_obj;
    //
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<enum_obj::Animal>(), 24);
        assert_eq!(std::mem::size_of::<trait_obj::Cat>(), 4);
        assert_eq!(std::mem::size_of::<trait_obj::Dog>(), 8);
        assert_eq!(std::mem::size_of::<trait_obj::Duck>(), 12);
        assert_eq!(std::mem::size_of::<trait_obj::Crow>(), 16);
        assert_eq!(std::mem::size_of::<trait_obj::Frog>(), 20);
        //
        assert_eq!(std::mem::size_of::<Box<enum_obj::Animal>>(), 8);
        assert_eq!(std::mem::size_of::<Box<dyn trait_obj::Animal>>(), 16);
        assert_eq!(std::mem::size_of::<Box<trait_obj::Cat>>(), 8);
        assert_eq!(std::mem::size_of::<Box<trait_obj::Dog>>(), 8);
        assert_eq!(std::mem::size_of::<Box<trait_obj::Duck>>(), 8);
        assert_eq!(std::mem::size_of::<Box<trait_obj::Crow>>(), 8);
        assert_eq!(std::mem::size_of::<Box<trait_obj::Frog>>(), 8);
    }
}
