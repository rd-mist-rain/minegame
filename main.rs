use std::io;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
fn create_map(map:&mut HashMap<(u32,u32),Point>,nmine:u32,nx:u32,ny:u32)->Vec<(u32,u32)>
{
    for x in 1..=nx
    {
        for y in 1..=ny
        {
            map.insert((x,y),Point{state:0,num:0});
        }
    }
    // 创建包含所有可能坐标的向量
    let mut points: Vec<(u32, u32)> = 
        (1..=nx).flat_map(|x| (1..=ny).map(move |y| (x, y)))
        .collect();

    // 打乱所有可能坐标的向量
    let mut rng = thread_rng();
    points.shuffle(&mut rng);

    // 生成不重复的随机坐标对
    let mines: Vec<(u32, u32)> = points.into_iter().take(nmine as usize).collect();

    for (mx,my) in mines.clone()
    {
        if let Some(point)=map.get_mut(&(mx,my))
        { 
            point.num=-1; 
            if let Some(p1)=map.get_mut(&(mx+1,my)) {p1.plus_one();}
            if let Some(p2)=map.get_mut(&(mx-1,my)) {p2.plus_one();}
            if let Some(p3)=map.get_mut(&(mx,my+1)) {p3.plus_one();}
            if let Some(p4)=map.get_mut(&(mx,my-1)) {p4.plus_one();}
            if let Some(p5)=map.get_mut(&(mx+1,my+1)) {p5.plus_one();}
            if let Some(p6)=map.get_mut(&(mx-1,my+1)) {p6.plus_one();}
            if let Some(p7)=map.get_mut(&(mx+1,my-1)) {p7.plus_one();}
            if let Some(p8)=map.get_mut(&(mx-1,my-1)) {p8.plus_one();}
        } 
    }
    return mines;
}
fn main()
{
    println!("Welcome to minegame");
    let mut map:HashMap<(u32,u32),Point>=HashMap::new();
    let nx=10;
    let ny=10;
    let nmine:u32=7;
    let mines=create_map(&mut map,nmine,nx,ny);
    /* 测试用代码，可以打印出所有雷的坐标
    for (i,j) in &mines
    {
        println!("{}",i);
        println!("{}",j);
    }
    */
    let mut winning:bool=true;
    print_map(&map,nx,ny);
    while winning==true {each(&mut map,&mut winning,&mines);print_map(&map,nx,ny);}
    println!("游戏结束,程序将在十秒后自动关闭");
    for i in 1..=ny
    {
        for j in 1..=nx
        {
            if let Some(point)=map.get(&(j,i))
            {
                if point.num!=-1
                {
                print!("{}",point.num);
                print!("  ");
                }
                else if point.num==-1
                {
                    print!("B");
                    print!("  ");
                }
            }
        }
        print!("\n");
    }
    std::thread::sleep(std::time::Duration::from_secs(10));

}
fn each(map: &mut HashMap<(u32,u32),Point>,winning:&mut bool,mines:&Vec<(u32,u32)>)
{
    let mut i=0;
    for &(cx,cy) in mines
    {
        if let Some(point)=map.get_mut(&(cx,cy))
        {
            if point.state==2{i+=1;}
        }
    }
    if i==mines.len()
    {
        println!("你获胜！");
        *winning=false;
        return;
    }
    let mut x_str=String::new();
    let mut y_str=String::new();
    let mut order=String::new();
    println!("请输入指令！(a:点击  f:标记)");
    io::stdin().read_line(&mut order).expect("Failed to read");
    match order.replace("\n","").replace("\r","").as_str()
    {
        "a"=>
        {
            println!("请输入点坐标");
            io::stdin().read_line(&mut x_str).expect("Failed to read");
            let x=x_str.trim().parse::<u32>().unwrap();
            io::stdin().read_line(&mut y_str).expect("Failed to read");
            let y=y_str.trim().parse::<u32>().unwrap();
            click(x,y,map,winning);
        }
        "f"=>
        {
            println!("请输入点坐标");
            io::stdin().read_line(&mut x_str).expect("Failed to read");
            let x=x_str.trim().parse::<u32>().unwrap();
            io::stdin().read_line(&mut y_str).expect("Failed to read");
            let y=y_str.trim().parse::<u32>().unwrap();
            flag(x,y,map)
        }
        e=>
        {
            println!("接收到了一条错误的指令{e}");
        }

    }
}
fn click(x: u32, y: u32, map: &mut HashMap<(u32, u32), Point>, winning: &mut bool) {
    let mut stack = vec![(x, y)];
    while let Some((cx, cy)) = stack.pop() {
        if let Some(point) = map.get_mut(&(cx, cy)) {
            if point.state == 0 
            {
                point.appear();
                if point.num == -1 
                {
                    *winning = false;
                    return;
                } 
                else if point.num == 0 {
                    stack.push((cx + 1, cy));
                    stack.push((cx - 1, cy));
                    stack.push((cx + 1, cy + 1));
                    stack.push((cx - 1, cy + 1));
                    stack.push((cx + 1, cy - 1));
                    stack.push((cx - 1, cy - 1));
                    stack.push((cx, cy - 1));
                    stack.push((cx, cy + 1));
                }
            }
        }
    }
}

fn flag(x:u32,y:u32,map:&mut HashMap<(u32,u32),Point>)
{
    if let Some(point)=map.get_mut(&(x,y))
    {
        if point.state==0
        {
            point.state=2;
        }
        else if point.state==2
        {
            point.state=0;
        }
    }
}
fn print_map(map:&HashMap<(u32,u32),Point>,nx:u32,ny:u32)
{
    for i in 1..=ny
    {
        for j in 1..=nx
        {
            if let Some(point)=map.get(&(j,i))
            {
                if point.state==0
                {
                    print!("/");
                    print!("  ");
                }
                else if point.state==1
                {
                    if point.num!=-1
                    {
                    print!("{}",point.num);
                    print!("  ");
                    }
                    else if point.num==-1
                    {
                        print!("B");
                        print!("  ");
                    }
                }
                else if point.state==2
                {
                    print!("F");
                    print!("  ");
                }
            }
        }
        print!("\n");
    }
}
#[derive(Debug)]
struct Point
{
    state:u8, //0:未翻开 1:翻开 2:标记
    num:i8, //-1:雷 0:空白 1~8:雷区
}
impl Point
{
    fn appear(&mut self){
    if self.state == 0 
    {
        self.state=1;
    }
    }
    fn plus_one(&mut self)
    {
        if self.num>=0 {self.num+=1;}
    }
}
