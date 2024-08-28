pub fn print_stuff() {
    #[cfg(feature = "foo")]
    {
        println!("foo");
    }

    #[cfg(feature = "bar")]
    {
        println!("bar");
    }
}
