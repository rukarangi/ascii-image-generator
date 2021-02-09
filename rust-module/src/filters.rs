pub fn grayscale_basic(strength: u32) -> char {
    let grayscale: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];

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

pub fn grayscale_detailed(strength: u32) -> char {
    let grayscale: [char; 68] = ['$','@','B','%','8','&','W','M','#','*','o','a','h','k','b','d','p','q','w','m','Z','O','0','Q','L','C','J','U','Y','X','z','c','v','u','n','x','r','j','f','t','/','|','(',')','1','{','}','[',']','?','-','_','+','~','<','>','i','!','l','I',';',':',',','^','`','.',',',' '];

    let mut character: char = ' ';

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
        100..=103 => grayscale[42],
        104..=107 => grayscale[41],
        108..=111 => grayscale[40],
        112..=115 => grayscale[39],
        116..=119 => grayscale[38],
        120..=123 => grayscale[37],
        124..=127 => grayscale[36],
        128..=131 => grayscale[35],
        132..=135 => grayscale[34],
        136..=139 => grayscale[33],
        140..=143 => grayscale[32],
        144..=147 => grayscale[31],
        148..=151 => grayscale[30],
        152..=155 => grayscale[29],
        156..=159 => grayscale[28],
        160..=163 => grayscale[27],
        164..=167 => grayscale[26],
        168..=171 => grayscale[25],
        172..=175 => grayscale[24],
        176..=179 => grayscale[23],
        180..=183 => grayscale[22],
        184..=187 => grayscale[21],
        188..=191 => grayscale[20],
        192..=195 => grayscale[19],
        196..=199 => grayscale[18],
        200..=203 => grayscale[17],
        204..=207 => grayscale[16],
        208..=211 => grayscale[15],
        212..=215 => grayscale[14],
        216..=219 => grayscale[13],
        220..=223 => grayscale[12],
        224..=227 => grayscale[11],
        228..=231 => grayscale[10],
        232..=235 => grayscale[9],
        236..=239 => grayscale[8],
        240..=243 => grayscale[7],
        244..=247 => grayscale[6],
        248..=251 => grayscale[5],
        252..=255 => grayscale[6],
        _ => grayscale[50]
    };

    return character;
}