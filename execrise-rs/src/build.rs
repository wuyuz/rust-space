
mod test {
    #[derive(Debug)]
    pub struct YakShaverBuilder {
        clipper_size: u32,
        gas_powered_clippers: bool,
        solar_powered_clippers: bool,
        color_to_dye_yak: String,
        clipper_color: String,
    }

    impl YakShaverBuilder {
        fn new() -> Self {   // 默认使用new方法创建实例，但是这种方式不太适配自定义，每次初始化构建实例
            Self {
                clipper_size: 3,
                gas_powered_clippers: false,
                solar_powered_clippers: true,
                color_to_dye_yak: String::from("brown"),
                clipper_color: String::from("black"),
            }
        }

        pub fn clipper_size(mut self, v: u32) -> Self {
            self.clipper_size = v;
            self
        }
    
        pub fn gas_powered_clippers(mut self, v: bool) -> Self {
            self.gas_powered_clippers = v;
            self
        }
    
        pub fn solar_powered_clippers(mut self, v: bool) -> Self {
            self.solar_powered_clippers = v;
            self
        }
    
        pub fn color_to_dye_yak(mut self, v: String) -> Self {
            self.color_to_dye_yak = v;
            self
        }
    
        pub fn clipper_color(mut self, v: String) -> Self {
            self.clipper_color = v;
            self
        }

        // build模式
        pub fn build(self) -> Self {
            YakShaverBuilder {
                clipper_size: self.clipper_size,
                gas_powered_clippers: self.gas_powered_clippers,
                solar_powered_clippers: self.solar_powered_clippers,
                color_to_dye_yak: self.color_to_dye_yak,
                clipper_color: self.clipper_color,
            }
        }
    }

    #[test]
    fn t_build() {
        let y = YakShaverBuilder::new()
            // .clipper_size(4)   // 没有就会取默认值
            .color_to_dye_yak(String::from("hot pink"))
            .clipper_color(String::from("red"))
            .build();

        println!("{:?}", y);
    }
}