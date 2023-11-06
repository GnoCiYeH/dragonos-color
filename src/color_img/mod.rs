use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

pub struct ColorImgPrinter<'a> {
    path: &'a str,
    tab_size: usize,
    extern_info: HashMap<usize, (u32, u32, String)>,
    max_line: usize,
}

impl<'a> ColorImgPrinter<'a> {
    /// ### 给一个gp2a工具生成的彩色txt文件，以及一个tab_size
    /// 返回一个可以将其彩色打印到DragonOS的对象
    pub fn new(path: &'a str, tab_size: usize) -> Self {
        Self {
            path,
            tab_size,
            extern_info: HashMap::new(),
            max_line: 0,
        }
    }

    pub fn tab_size(&self) -> usize {
        self.tab_size
    }

    pub fn set_tab_size(&mut self, size:usize) -> &mut Self{
        self.tab_size = size;
        self
    }

    /// ### 注册一个额外信息，可以在打印字符画的同时将额外信息打印在右侧
    pub fn registe_info(&mut self, line_num: usize, front: u32, back: u32, info: String) {
        self.max_line += 1;
        self.extern_info.insert(line_num, (front,back,info));
    }

    /// ### 打印
    pub fn print_color(&mut self) {
        let mut buf = Vec::new();
        let mut tmp = String::new();
        let mut reader = BufReader::new(File::open(self.path).unwrap());
        reader.read_to_string(&mut tmp).unwrap();

        let lines = tmp.split("\n").collect::<Vec<_>>();

        if lines.len() > self.max_line {
            self.max_line = lines.len();
        }

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let ret = line.split("[0m[38;2;").collect::<Vec<_>>();
            buf.extend(ret);
            buf.push("\n");
        }

        let mut line_num = 0;
        for pat in buf {
            if pat.is_empty() {
                continue;
            }
            if pat == "\n" {
                self.print_extern(line_num);
                println!();
                line_num += 1;
                continue;
            }
            let pat = pat.replace("[0m", "");
            let ret = pat.split(";").collect::<Vec<_>>();
            let red = ret[0].parse::<u32>().unwrap_or(0);
            let green = ret[1].parse::<u32>().unwrap_or(0);
            let tmp = ret[2].splitn(2, "m").collect::<Vec<_>>();
            let blue = tmp[0].parse::<u32>().unwrap_or(0);

            let color = (red << 16) | (green << 8) | blue;
            let mut s = tmp[1].to_string();
            s.push('\0');
            unsafe { libc::syscall(100000, s.as_ptr(), u64::from(color), 0) };
        }
        if line_num < self.max_line {
            for _ in line_num..self.max_line {
                self.print_extern(line_num);
                println!();
            }
        }
    }

    /// ### 输出额外信息
    fn print_extern(&self, line_num: usize) {
        if let Some((front, back, info)) = self.extern_info.get(&line_num) {
            for _ in 0..self.tab_size {
                print!("\t");
            }
            let mut info = info.clone();
            info.push('\0');
            unsafe { libc::syscall(100000, info.as_ptr(), u64::from(*front), u64::from(*back)) };
        }
    }
}
