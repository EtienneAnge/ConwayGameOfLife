use std::mem;

fn main() {


    const SCREEN_HEIGHT: usize = 40;
    const SCREEN_WIDTH: usize = 70;
    let mut grid = [[false; SCREEN_WIDTH];SCREEN_HEIGHT];
    let mut nextGrid= [[false; SCREEN_WIDTH];SCREEN_HEIGHT];    
grid[20][21] = true; grid[20][22] = true;
grid[21][20] = true; grid[21][21] = true;
grid[22][21] = true;
    
    loop{
        screenRefresh(&grid);
        iteration(&grid, &mut nextGrid);
        mem::swap(&mut grid, &mut nextGrid);
    
        std::thread::sleep(std::time::Duration::from_millis(100));
    }


}

fn iteration<const H: usize, const W: usize>(
    grid : &[[bool; W];H],
    nextGrid: &mut [[bool;W];H]
){
    
    for i in 0..H{
        for j in 0..W {
            nextGrid[i][j] = newCell(i,j,grid);
            
        }
    }

    

}

fn newCell<const H: usize, const W: usize>(i: usize, j: usize, grid : &[[bool; W];H]) -> bool{
    let mut numberOfTrueEdgeCell = 0;
    let i = i as isize;
    let j = j as isize;

    for k in -1..=1{
        for l in -1..=1{
            let m = i+k;
            let n = j+l;
            if(m < 0 || n < 0 || m >= H as isize || n >= W as isize){
                continue;
            }
            if grid[m as usize][n as usize]{
                numberOfTrueEdgeCell+=1;
            } 
                
        }
    }



    if grid[i as usize][j as usize]{
        numberOfTrueEdgeCell-=1;
        if(numberOfTrueEdgeCell == 2 || numberOfTrueEdgeCell == 3){
            return true
        }
    }else {
        if(numberOfTrueEdgeCell == 3){
            return true
        }
    }
    false
    
}

fn screenRefresh<const H: usize, const W: usize>(grid : &[[bool; W];H]){
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    for row in grid {
        let mut line : String = String::new();
        for cell in row {
            line.push_str( match cell {
                true => "██",
                false => "  " 
            });
        }
        println!("{line}|");
    }
}