pub fn egg_count(display_value: u32) -> usize {
    // display_value.count_ones() as usize
    /*
     * 解法一： 转换字符串处理
     */
    // format!("{:b}", display_value)
    //     .chars()
    //     .filter(|c| *c == '1')
    //     .count()

    /*
     * 解法二： 位运算
     */
    let mut count = 0;
    let mut value = display_value;
    while value > 0 {
        // 检查最低位是否为1
        count += value & 1;
        // 右移一位
        value >>= 1;
    }
    count as usize
}
