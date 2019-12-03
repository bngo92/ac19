use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args[1] == "p1" {
        println!("{}", d1(12));
        println!("{}", d1(14));
        println!("{}", d1(1969));
        println!("{}", d1(100756));
        let i: i32 = input().into_iter().map(d1).sum();
        println!("{}", i);
    } else if args[1] == "p2" {
        println!("{}", S { state: 14 }.sum::<i32>());
        println!("{}", S { state: 1969 }.sum::<i32>());
        println!("{}", S { state: 100756 }.sum::<i32>());
        let i: i32 = input()
            .into_iter()
            .map(|s| S { state: s }.sum::<i32>())
            .sum();
        println!("{}", i);
    }
}

struct S {
    state: i32,
}

impl Iterator for S {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.state = d1(self.state);
        if self.state > 0 {
            Some(self.state)
        } else {
            None
        }
    }
}
pub fn d1(input: i32) -> i32 {
    input / 3 - 2
}

pub fn input() -> Vec<i32> {
    let input = "71764
58877
107994
72251
74966
87584
118260
144961
86889
136710
52493
131045
101496
124341
71936
88967
106520
125454
113463
81854
99918
105217
120383
61105
103842
125151
139191
143365
102168
69845
57343
93401
140910
121997
107964
53358
57397
141456
94052
127395
99180
143838
130749
126809
70165
92007
83343
55163
95270
101323
99877
105721
129657
61213
130120
108549
90539
111382
61665
95121
53216
103144
134367
101251
105118
73220
56270
50846
77314
59134
98495
113654
89711
68676
98991
109068
129630
58999
132095
98685
91762
88589
73846
124940
106944
133882
104073
78475
76545
144728
72449
118320
65363
83523
124634
96222
128252
112848
139027
108208";
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
