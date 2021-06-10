
mod test {
    fn show_len(s: &str) -> usize {
        s.len()
    }

    async fn str_len_async(s: &str) -> usize {
        // do something awaitable ideally... 
        s.len()
    }

    // 方式一
    #[actix_rt::test]
    async fn t_actix() {   // 本身是异步函数
        assert_eq!(show_len("111d"),4);
        assert_eq!(str_len_async("222").await,3);
    }

    // 方式二
    #[test]
    fn t_tokio() {
        use tokio_test::block_on;
        assert_eq!(block_on(str_len_async("ww")),2)
    }

}