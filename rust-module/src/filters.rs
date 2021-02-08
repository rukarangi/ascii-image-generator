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