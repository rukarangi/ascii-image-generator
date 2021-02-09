pub fn grayscale_basic(strength: u32, reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec![' ','.',':','-','=','+','*','#','%','@'];
    let grayscale: Vec<char>;

    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    let character: char;

    if strength < 25 {
        character = grayscale[0];
    } else if strength < 50 {
        character = grayscale[1];
    } else if strength < 75 {
        character = grayscale[2];
    } else if strength < 100 {
        character = grayscale[3];
    } else if strength < 125 {
        character = grayscale[4];
    } else if strength < 150 {
        character = grayscale[5];
    } else if strength < 175 {
        character = grayscale[6];
    } else if strength < 200 {
        character = grayscale[7];
    } else if strength < 225 {
        character = grayscale[8];
    } else {
        character = grayscale[9];
    }

    return character;
}

pub fn grayscale_basic_reverse(strength: u32) -> char {
    let grayscale: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];

    let character: char;

    if strength < 25 {
        character = grayscale[9];
    } else if strength < 50 {
        character = grayscale[8];
    } else if strength < 75 {
        character = grayscale[7];
    } else if strength < 100 {
        character = grayscale[6];
    } else if strength < 125 {
        character = grayscale[5];
    } else if strength < 150 {
        character = grayscale[4];
    } else if strength < 175 {
        character = grayscale[3];
    } else if strength < 200 {
        character = grayscale[2];
    } else if strength < 225 {
        character = grayscale[1];
    } else {
        character = grayscale[0];
    }

    return character;
}

pub fn grayscale_detailed(strength: u32, reverse: bool) -> char {
    let grayscale_default: Vec<char> = vec!['$','@','B','%','8','&','W','M','#','*','o','a','h','k','b','d','p','q','w','m','Z','O','0','Q','L','C','J','U','Y','X','z','c','v','u','n','x','r','j','f','t','/','|','(',')','1','{','}','[',']','?','-','_','+','~','<','>','i','!','l','I',';',':',',','^','`','.',',',' '];
    let grayscale: Vec<char>;

    if reverse {
        grayscale = grayscale_default.into_iter().rev().collect();
    } else {
        grayscale = grayscale_default;
    }

    let character: char;

    match strength {
        0..=3 => character = grayscale[67],
        4..=7 => character = grayscale[66],
        8..=11 => character = grayscale[65],
        12..=15 => character = grayscale[64],
        16..=19 => character = grayscale[63],
        20..=23 => character = grayscale[62],
        24..=27 => character = grayscale[61],
        28..=31 => character = grayscale[60],
        32..=35 => character = grayscale[59],
        36..=39 => character = grayscale[58],
        40..=43 => character = grayscale[57],
        44..=47 => character = grayscale[56],
        48..=51 => character = grayscale[55],
        52..=55 => character = grayscale[54],
        56..=59 => character = grayscale[53],
        60..=63 => character = grayscale[52],
        64..=67 => character = grayscale[51],
        68..=71 => character = grayscale[50],
        72..=75 => character = grayscale[49],
        76..=79 => character = grayscale[48],
        80..=83 => character = grayscale[47],
        84..=87 => character = grayscale[46],
        88..=91 => character = grayscale[45],
        92..=95 => character = grayscale[44],
        96..=99 => character = grayscale[43],
        100..=103 => character = grayscale[42],
        104..=107 => character = grayscale[41],
        108..=111 => character = grayscale[40],
        112..=115 => character = grayscale[39],
        116..=119 => character = grayscale[38],
        120..=123 => character = grayscale[37],
        124..=127 => character = grayscale[36],
        128..=131 => character = grayscale[35],
        132..=135 => character = grayscale[34],
        136..=139 => character = grayscale[33],
        140..=143 => character = grayscale[32],
        144..=147 => character = grayscale[31],
        148..=151 => character = grayscale[30],
        152..=155 => character = grayscale[29],
        156..=159 => character = grayscale[28],
        160..=163 => character = grayscale[27],
        164..=167 => character = grayscale[26],
        168..=171 => character = grayscale[25],
        172..=175 => character = grayscale[24],
        176..=179 => character = grayscale[23],
        180..=183 => character = grayscale[22],
        184..=187 => character = grayscale[21],
        188..=191 => character = grayscale[20],
        192..=195 => character = grayscale[19],
        196..=199 => character = grayscale[18],
        200..=203 => character = grayscale[17],
        204..=207 => character = grayscale[16],
        208..=211 => character = grayscale[15],
        212..=215 => character = grayscale[14],
        216..=219 => character = grayscale[13],
        220..=223 => character = grayscale[12],
        224..=227 => character = grayscale[11],
        228..=231 => character = grayscale[10],
        232..=235 => character = grayscale[9],
        236..=239 => character = grayscale[8],
        240..=243 => character = grayscale[7],
        244..=247 => character = grayscale[6],
        248..=251 => character = grayscale[5],
        252..=255 => character = grayscale[0],
        _ => character = grayscale[50]
    };

    return character;
}