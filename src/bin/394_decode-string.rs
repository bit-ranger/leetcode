//给定一个经过编码的字符串，返回它解码后的字符串。
//
// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
//
// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
//
// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
//
// 示例:
//
//
//s = "3[a]2[bc]", 返回 "aaabcbc".
//s = "3[a2[c]]", 返回 "accaccacc".
//s = "2[abc]3[cd]ef", 返回 "abcabccdcdcdef".
//
// Related Topics 栈 深度优先搜索




//leetcode submit region begin(Prohibit modification and deletion)
pub fn decode_string(s: String) -> String {
    let mut rst = String::new();
    let mut stack = Vec::new();
    let mut multi = 0;

    let chars:Vec<char> = s.chars().collect();
    for c in chars{
        if c == '['{
            stack.push((rst, multi));
            rst = String::new();
            multi = 0;
        } else if c == ']'{
            let  (p_rst, p_multi) = stack.pop().unwrap();
            let mrst = rst.repeat(p_multi);
            rst = String::new();
            rst.push_str(p_rst.as_str());
            rst.push_str(mrst.as_str());
        } else if '0' <= c && c <= '9'{
            multi = multi*10 + c.to_digit(10).unwrap() as usize;
        } else {
            rst.push(c)
        }
    }

    return rst;
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn decode_string_test() {
    let rst = decode_string(String::from("3[a2[c]]"));
    assert_eq!(rst, String::from("accaccacc"));
}