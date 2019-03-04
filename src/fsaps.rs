pub fn read_line() -> String {
    let mut return_ = format!("");
    std::io::stdin().read_line(&mut return_).ok();
    return_.pop();
    return_
}

pub fn kmp_table(pattern: &[u8], m: usize, mut kmp: Vec<usize>) -> Vec<usize> {
    let mut len: usize = 0;
    kmp[0] = 0;
    let mut i: usize = 1;
    while i < m {
        if pattern[i] == pattern[len] {
            len += 1;
            kmp[i] = len;
            i += 1;
        } else if len != 0 {
            len = kmp[len - 1];
        } else {
            kmp[i] = 0;
            i += 1;
        }
    }
    kmp
}

pub fn kmp(string: &Vec<u8>, pattern: &Vec<u8>) {
    let mut out: String = String::new();
    let n = string.len();
    let m = pattern.len();
    let kmp = kmp_table(&pattern, m, vec![0usize; m]);
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < n {
        if pattern[j] == string[i] {
            i = i + 1;
            j = j + 1;
        }
        if j == m {
            out.push_str(&(i - j).to_string());
            out.push('\n');
            j = kmp[j - 1];
        } else if i < n && pattern[j] != string[i] {
            if j != 0 {
                j = kmp[j - 1];
            } else {
                i = i + 1;
            }
        }
    }
    print!("{}", out);
}
