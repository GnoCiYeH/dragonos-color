use std::{collections::HashMap, fmt::Display};

use crate::DRAGONOS_PUT_STRING;

/// ### 渲染字体颜色的渲染器
pub struct Renderer<'a> {
    char_color: HashMap<char, (usize, usize)>,
    string_color: HashMap<&'a str, (usize, usize)>,
    default_color: (usize, usize),
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self {
            char_color: HashMap::new(),
            string_color: HashMap::new(),
            default_color: (0xffffffusize, 0usize), // 默认白字黑底
        }
    }

    pub fn set_default(&mut self, front_color: usize, back_color: usize) -> &mut Self {
        self.default_color = (front_color, back_color);
        self
    }

    /// ### 注册一个字符到渲染器，后续该字符将会以设置的颜色显示
    ///
    /// 优先级 char < string ，因此，与字符串渲染冲突时优先考虑字符串
    pub fn register_char(&mut self, ch: char, front_color: usize, back_color: usize) -> &mut Self {
        self.char_color.insert(ch, (front_color, back_color));
        self
    }

    /// ### 注册一个字符串到渲染器
    pub fn register_string(
        &mut self,
        word: &'a str,
        front_color: usize,
        back_color: usize,
    ) -> &mut Self {
        self.string_color.insert(word, (front_color, back_color));
        self
    }

    /// ### 取消注册
    pub fn unregister_char(&mut self, ch: char) {
        self.char_color.remove(&ch);
    }

    /// ### 取消注册
    pub fn unregister_string(&mut self, word: &'a str) {
        self.string_color.remove(&word);
    }

    pub fn unregister_all_char(&mut self) {
        self.char_color.clear();
    }

    pub fn unregister_all_string(&mut self) {
        self.string_color.clear();
    }

    /// ### 渲染输出
    pub fn render<A: Display>(&self, src: A) {
        // 对应index的颜色
        let src = src.to_string();
        let mut color_map: HashMap<usize, (usize, usize)> = HashMap::new();
        for (index, ch) in src.char_indices() {
            if let Some(color) = self.char_color.get(&ch) {
                // 有与之对应的颜色，则设置
                color_map.insert(index, *color);
            }
        }

        for (word, color) in self.string_color.iter() {
            let indexs = Self::get_sub_string_indexs(&src, *word);
            for index in indexs {
                // 将该字符串的所有index设置对应颜色
                for i in index..index + word.len() {
                    color_map.insert(i, *color);
                }
            }
        }

        let mut start = 0;
        let len = src.len();
        while start < len {
            let mut tmp = String::new();
            let color = color_map.get(&start).unwrap_or(&self.default_color);
            for index in start..len {
                tmp.push(src.chars().nth(index).unwrap());
                if index + 1 < len {
                    let next_color = color_map.get(&(index + 1)).unwrap_or(&self.default_color);
                    if *next_color != *color {
                        start += 1;
                        break;
                    }
                }
                start += 1;
            }
            tmp.push('\0');
            unsafe {
                libc::syscall(DRAGONOS_PUT_STRING, tmp.as_ptr(), color.0, color.1);
            }
        }
    }

    pub fn renderln<A: Display>(&self, src: A) {
        self.render(src);
        println!()
    }

    /// ### 获取一个字符串中所有目标子字符串的起始坐标
    fn get_sub_string_indexs(src: &String, word: &'a str) -> Vec<usize> {
        let mut indexs = Vec::new();
        let mut start = 0;
        while let Some(index) = src[start..].find(word) {
            let absolute_index = start + index;
            indexs.push(absolute_index);
            start = absolute_index + 1;
        }
        indexs
    }

    /// ### 以对应颜色打印
    pub fn render_print<A: Display>(info: A, front_color: usize, back_color: usize) {
        let mut info = info.to_string();
        info.push('\0');
        unsafe {
            libc::syscall(DRAGONOS_PUT_STRING, info.as_ptr(), front_color, back_color);
        }
    }

    /// ### 以对应颜色打印
    pub fn render_println<A: Display>(info: A, front_color: usize, back_color: usize) {
        let mut info = info.to_string();
        info.push_str("\n\0");
        unsafe {
            libc::syscall(DRAGONOS_PUT_STRING, info.as_ptr(), front_color, back_color);
        }
    }
}

#[test]
fn test() {
    use self::*;

    let mut render = Renderer::new();
    render.register_char('o', 0xff0000, 0);
    render.register_char('b', 0x00ff00, 0);
    render.register_char('c', 0x0000ff, 0);
    render.register_string("hello", 0xF0FFF0, 0);
    render.render("hello dragonos color lib");

    Renderer::render_println("hello dragonos color lib", 0, 0);
}
