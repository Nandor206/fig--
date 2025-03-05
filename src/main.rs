use clap::Parser;

#[derive(Parser)]
#[command(about = "A CLI tool that converts text to ASCII art")]
struct Cli {
    #[arg(help = "The text to convert to ASCII art")]
    text: Option<String>,
}

const ASCII_ART: [&str; 95] = [
    "     \n     \n     \n     \n     ", // ASCII 32 (Space)
    "  !  \n  !  \n  !  \n     \n  !  ", // ASCII 33 (!)
    "  \" \n  \" \n     \n     \n     ", // ASCII 34 (")
    " # # \n#####\n # # \n#####\n # # ", // ASCII 35 (#)
    "  $  \n $$$$\n$ $  \n $$$$\n  $  ", // ASCII 36 ($)
    "  %  \n%   %\n  %  \n%   %\n  %  ", // ASCII 37 (%)
    "  &  \n & & \n  &  \n & & \n  &  ", // ASCII 38 (&)
    "  '  \n     \n     \n     \n     ", // ASCII 39 (')
    "  (  \n (   \n (   \n  (  \n     ", // ASCII 40 ('(')
    "  )  \n   ) \n   ) \n  )  \n     ", // ASCII 41 (')')
    "  *  \n * * \n*****\n * * \n  *  ", // ASCII 42 ('*')
    "     \n  +  \n +++ \n  +  \n     ", // ASCII 43 ('+')
    "     \n     \n     \n  ,  \n ,   ", // ASCII 44 (',')
    "     \n     \n --- \n     \n     ", // ASCII 45 ('-')
    "     \n     \n     \n  .  \n     ", // ASCII 46 ('.')
    "    /\n   / \n  /  \n /   \n     ", // ASCII 47 ('/')
    " 000 \n0   0\n0   0\n0   0\n 000 ", // ASCII 48 ('0')
    "  1  \n 11  \n  1  \n  1  \n 111 ", // ASCII 49 ('1')
    " 222 \n2   2\n   2 \n  2  \n 2222", // ASCII 50 ('2')
    " 333 \n    3\n 333 \n    3\n 333 ", // ASCII 51 ('3')
    " 4 4 \n 4 4 \n 4444 \n   4 \n   4 ", // ASCII 52 ('4')
    " 5555\n 5   \n 555 \n    5\n 555 ", // ASCII 53 ('5')
    " 666 \n 6   \n 666 \n 6  6\n 666 ", // ASCII 54 ('6')
    " 7777\n    7\n   7 \n  7  \n 7   ", // ASCII 55 ('7')
    " 888 \n8   8\n 888 \n8   8\n 888 ", // ASCII 56 ('8')
    " 999 \n9   9\n 9999\n    9\n 999 ", // ASCII 57 ('9')
    "  :  \n     \n  :  \n     \n     ", // ASCII 58 (':')
    "  ;  \n     \n  ;  \n  ;  \n     ", // ASCII 59 (';')
    "   < \n  <  \n <   \n  <  \n   < ", // ASCII 60 ('<')
    "     \n === \n     \n === \n     ", // ASCII 61 ('=')
    " >   \n  >  \n   > \n  >  \n >   ", // ASCII 62 ('>')
    " ??? \n    ?\n  ?? \n     \n  ?  ", // ASCII 63 ('?')
    "  @  \n @@@ \n@ @ @\n @@@ \n  @  ", // ASCII 64 ('@')
    "  A  \n A A \nAAAAA\nA   A\nA   A", // ASCII 65 ('A')
    "BBBB \nB   B\nBBBB \nB   B\nBBBB ", // ASCII 66 ('B')
    "  C  \n C  C\n C  C\n   C \n C  C", // ASCII 67 ('C')
    " DDD \nD   D\nD   D\nD   D\nDDDD ", // ASCII 68 ('D')
    "  E  \n E  E\n E  E\n E   \n E  E", // ASCII 69 ('E')
    " FFF \n F   F\n F   F\n F   F\n FFF ", // ASCII 70 ('F')
    " GGG \nG   G\nG   G\n GGGG\n   G ", // ASCII 71 ('G')
    "  H  \n H   \n HHHH \n H   \n H   ", // ASCII 72 ('H')
    " II  \n I   \n I   \n I   \n II  ", // ASCII 73 ('I')
    " JJJ \n    J\n    J\n JJJJ\n    J ", // ASCII 74 ('J')
    "  K  \n K  K\n  K  \n K  K\n K  K ", // ASCII 75 ('K')
    "  L  \n L   \n L   \n L   L\n LLL ", // ASCII 76 ('L')
    " MMMM \nM   M\n M   M\n MMMM \n M   M", // ASCII 77 ('M')
    " NNNN \nN   N\nN   N\n NNNN \n N   N", // ASCII 78 ('N')
    " OOO \n O   O\n O   O\n O   O\n OOO ", // ASCII 79 ('O')
    " PPPP \nP   P\n PPPP \n    P\n PPPP ", // ASCII 80 ('P')
    " QQQQ \nQ   Q\nQ   Q\nQQQQQ\n Q   Q", // ASCII 81 ('Q')
    " RRRR \nR   R\nRRRR \n R   R\n R  R ", // ASCII 82 ('R')
    " SSSS \nS   S\n SSSS \n     S\n SSSS ", // ASCII 83 ('S')
    " TTT \n T   T\n     T\n     T\n TTT ", // ASCII 84 ('T')
    " UUUU \nU   U\n U   U\n U   U\n UUUU ", // ASCII 85 ('U')
    " VVVV \n V   V\n V   V\n     V\n     V ", // ASCII 86 ('V')
    " WWWW \n W   W\n W   W\n WWWW \n W   W ", // ASCII 87 ('W')
    " X XX \nX   X\n     X\n     X\n X XX ", // ASCII 88 ('X')
    " YYY \n Y   Y\n     Y\n Y   Y\n YYY ", // ASCII 89 ('Y')
    " ZZZZ \n Z   Z\n Z   Z\n ZZZZ \n Z   Z ", // ASCII 90 ('Z')
    "  [  \n  [  \n  [  \n  [  \n  [  ", // ASCII 91 ('[')
    "  \\  \n  \\  \n  \\  \n  \\  \n  \\  ", // ASCII 92 ('\')
    "  ]  \n  ]  \n  ]  \n  ]  \n  ]  ", // ASCII 93 (']')
    "  ^  \n  ^  \n  ^  \n  ^  \n  ^  ", // ASCII 94 ('^')
    "  _  \n  _  \n  _  \n  _  \n  _  ", // ASCII 95 ('_')
    "  `  \n  `  \n  `  \n  `  \n  `  ", // ASCII 96 ('`')
    "  a  \n a a \n a a \n a a \n a a ", // ASCII 97 ('a')
    " bbb \n b   b\n bbb \n b   b\n bbb ", // ASCII 98 ('b')
    "  c  \n c  c\n c  c\n   c \n  c  ", // ASCII 99 ('c')
    " ddd \n d   d\n ddd \n     d\n ddd ", // ASCII 100 ('d')
    "  e  \n e  e\n e  e\n e   \n e  e", // ASCII 101 ('e')
    " fff \n f   f\n f   f\n f   f\n fff ", // ASCII 102 ('f')
    " ggg \ng   g\ng   g\n gggg\n   g ", // ASCII 103 ('g')
    "  h  \n h   \n hhhh \n h   \n h   ", // ASCII 104 ('h')
    " ii  \n i   \n i   \n i   \n ii  ", // ASCII 105 ('i')
    " jjj \n    j\n    j\n jjjj\n    j ", // ASCII 106 ('j')
    "  k  \n K  k\n  k  \n k  k\n k  k ", // ASCII 107 ('k')
    "  l  \n l   \n l   \n l   l\n lll ", // ASCII 108 ('l')
    " mmmm \nm   m\n m   m\n mmmm \n m   m", // ASCII 109 ('m')
    " nnnn \nn   n\nn   n\n nnnn \n n   n", // ASCII 110 ('n')
    " ooo \n o   o\n o   o\n o   o\n ooo ", // ASCII 111 ('o')
    " pppp \np   p\n pppp \n    p\n pppp ", // ASCII 112 ('p')
    " qqqq \nq   q\nq   q\nqqqqq\n q   q", // ASCII 113 ('q')
    " rrrr \nr   r\nrrrr \n r   r\n r  r ", // ASCII 114 ('r')
    " ssss \ns   s\n ssss \n     s\n ssss ", // ASCII 115 ('s')
    " ttt \n t   t\n     t\n     t\n ttt ", // ASCII 116 ('t')
    " uuuu \nu   u\n u   u\n u   u\n uuuu ", // ASCII 117 ('u')
    " vvvv \n v   v\n v   v\n     v\n     v ", // ASCII 118 ('v')
    " wwww \n w   w\n w   w\n wwww \n w   w ", // ASCII 119 ('w')
    " x xx \nx   x\n     x\n     x\n x xx ", // ASCII 120 ('x')
    " yyy \n y   y\n     y\n y   y\n yyy ", // ASCII 121 ('y')
    "  zzz\n     z\n  zzz\n z    \nzzzzz", // ASCII 122 ('z')
    "  {  \n  {  \n  {  \n  {  \n  {  ", // ASCII 123 ('{')
    "  |  \n  |  \n  |  \n  |  \n  |  ", // ASCII 124 ('|')
    "  }  \n  }  \n  }  \n  }  \n  }  ", // ASCII 125 ('}')
    "  ~  \n  ~  \n  ~  \n  ~  \n  ~  "  // ASCII 126 ('~')
];


fn main() {
    let args = Cli::parse();
    let text = args.text.unwrap_or_else(|| "fig++".to_string());

    art(&text);
}

fn art(text: &str) {
    let lenght = text.len();
    let mut ascii_nums = Vec::new();
    for c in text.chars() {
        ascii_nums.push((c as u32) - 32);
    }
    for row in 0..5 {
        for n in 0..lenght {
            let ascii_index = ascii_nums[n] as usize;
            if ascii_index < ASCII_ART.len() {
                let art_line = ASCII_ART[ascii_index].lines().nth(row).unwrap_or("");
                print!("{: <5}",art_line);
            }
        }
        println!();
    }
}
