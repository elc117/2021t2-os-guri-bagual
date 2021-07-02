use std::collections::LinkedList;

fn show_table(lines: &mut [[i8; 9]; 9])
{
    let mut newline = String::new();
    for _c in 0..(9*3+2)                    //Preencher 9 traços vezes 3 que é a quantidade de caracteres que cada quadrado utiliza
    {                                       //E mais 2 por causa dos dois espaços usados pra separar os blocos 3x3 do tabuleiro
        newline.push('-');
    }
    for x in 0..9
    {
        for y in 0..9
        {
            if y % 3 == 0 && y != 0         //Separar os blocos 3x3 horizontalmente
            {
                print!(" ");
            }
            print!("|{}|", lines[x][y]);    //Escolhi printar os espaços em branco como 0 para facilitar a visualização
        }
        println!("\n{}", newline);
        if (x+1) % 3 == 0                   //Separar os blocos 3x3 verticalmente
        {
            print!("\n");
        }
    }
}

//Testes
fn valid_line(lines: &mut [[i8; 9]; 9], line: usize) -> bool                //Se a linha tiver numeros repetidos retorna falso
{
    let mut numbers: LinkedList<i8> = LinkedList::new();            //Usar lista pela função contains
    for y in 0..9
    {
        if lines[line][y] != 0 && numbers.contains(&lines[line][y])
        {
            return false;
        }
        else if lines[line][y] != 0
        {
            numbers.push_back(lines[line][y]);
        }
    }
    return true;
}

fn valid_column(lines: &mut [[i8; 9]; 9], column: usize) -> bool            //Se a coluna tiver numeros repetidos retorna falso
{
    let mut numbers: LinkedList<i8> = LinkedList::new();
    for x in 0..9
    {
        if lines[x][column] != 0 && numbers.contains(&lines[x][column])
        {
            return false;
        }
        else if lines[x][column] != 0
        {
            numbers.push_back(lines[x][column]);
        }
    }
    return true;
}
/*
Classiicaçoes dos blocos:
    0         1         2
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
    3         4         5
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
    6         7         8
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
|0||0||0| |0||0||0| |0||0||0|
-----------------------------
*/

fn block(line: i8, column: i8) -> i8
{
    let sectorx: i8 = line / 3;
    let sectory: i8 = 3 * (column / 3);
    let sector: i8 = sectorx + sectory;
    return sector;
}

fn valid_block(lines: &mut [[i8; 9]; 9], block: i8) -> bool            //Se tiver numeros repetidos no bloco retorna falso
{
    let mut numbers: LinkedList<i8> = LinkedList::new();
    let mut line: usize = 3 * (block % 3) as usize;
    let mut column: usize = 3 * (block / 3) as usize;
    for x in line..(line+3)
    {
        for y in column..(column+3)
        {
            if lines[x][y] != 0 && numbers.contains(&lines[x][y])
            {
                return false;
            }
            else if lines[x][y] != 0
            {
                numbers.push_back(lines[x][y]);
            }
        }
    }
    return true;
}

fn find_empty_square(lines: &mut [[i8; 9]; 9], original_lines: [[i8;9]; 9]) -> (usize,usize)
{
    for x in 0..9
    {
        for y in 0..9
        {
            if lines[x][y] == 0
            {
                return (x,y);
            }
        }
    }
    return (10,10);
}

fn insert_number(lines: &mut [[i8; 9]; 9], (x,y): (usize,usize), movements: &mut Vec<i8>) -> bool
{
    let number = lines[x][y];
    for insertion in (number+1)..10      //Insere um numero de 1 a 9
    {
        lines[x][y] = insertion;
        println!("{} {}", x,y);
        show_table(lines);
        if valid_line(lines, x)
        {
            if valid_column(lines, y)
            {
                if valid_block(lines, block(x as i8,y as i8))
                {
                    movements.push(x as i8);
                    movements.push(y as i8);
                    return true;
                }
            }
        }
    }
    lines[x][y] = 0;
    return false;
}

fn modify_number(lines: &mut [[i8; 9]; 9], movements: &mut Vec<i8>) -> bool
{
    if movements.len() == 0
    {
        println!("eioa");
        return false;
    }
    let y = movements.pop().unwrap();
    let x = movements.pop().unwrap();
    if !insert_number(lines, (x as usize,y as usize), movements)
    {
        return modify_number(lines, movements);
    }
    return true;
}

fn backtracking(lines: &mut [[i8; 9]; 9], original_lines: [[i8;9]; 9], movements: &mut Vec<i8>) -> bool
{
    let (x,y) = find_empty_square(lines, original_lines); // tuple = (x,y)
    if x == 10
    {
        return true;
    }
    if !insert_number(lines, (x,y), movements)
    {
        if !modify_number(lines, movements)
        {
            return false;
        }
    }
    return backtracking(lines, original_lines, movements);
}

fn main()
{
    let mut lines: [[i8; 9]; 9] = [[0;9];9]; //9 Arrays de 9 elementos com inteiros de 8 bits, já que só usaremos numeros de 0 a 9
    let mut table: [[i8; 9]; 9] = [[0;9];9]; //A tabela inicial para sabermos qual quadrado o programa nao pode alterar
    let mut stack: Vec<i8> = Vec::new();
    table[0][2] = 7;
    table[0][3] = 6;
    table[0][6] = 3;
    table[0][8] = 4;
    table[1][2] = 4;
    table[1][5] = 2;
    table[1][8] = 8;
    table[2][3] = 4;
    table[2][4] = 7;
    table[2][5] = 3;
    table[2][7] = 6;
    table[3][0] = 9;
    table[3][4] = 1;
    table[4][0] = 2;
    table[4][2] = 5;
    table[4][6] = 8;
    table[4][8] = 9;
    table[5][4] = 5;
    table[5][8] = 7;
    table[6][1] = 6;
    table[6][3] = 9;
    table[6][4] = 4;
    table[6][5] = 1;
    table[7][0] = 7;
    table[7][3] = 5;
    table[7][6] = 4;
    table[8][0] = 4;
    table[8][2] = 9;
    table[8][5] = 8;
    table[8][6] = 2;
    lines = table;
    backtracking(&mut lines, table, &mut stack);
    //show_table(&mut lines);
}