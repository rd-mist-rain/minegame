// test7.1
#include <iostream>
#include <random>
#include <vector>
using namespace std;

vector <vector<int>> board; //-1为雷
vector <vector<int>> appear; //0隐藏，1显示，2标记

// 初始化地图，设置雷和数字
void defmap(int num, int row, int column)
{
    board.resize(row, vector<int>(column, 0));
    appear.resize(row, vector<int>(column, 0));
    random_device rd;
    mt19937 gen(rd());
    uniform_int_distribution<> dis_row(0, row - 1);
    uniform_int_distribution<> dis_col(0, column - 1);

    for(int i = 0; i < num; i++)
    {
        int a, b;
        do 
        {
            a = dis_row(gen);
            b = dis_col(gen);
        } 
        while (board[a][b] == -1);

        board[a][b] = -1;

        // 更新相邻单元格的值，增加边界检查
        if (a + 1 < row && board[a + 1][b] != -1) 
            board[a + 1][b] += 1;
        if (a - 1 >= 0 && board[a - 1][b] != -1) 
            board[a - 1][b] += 1;
        if (b + 1 < column && board[a][b + 1] != -1) 
            board[a][b + 1] += 1;
        if (b - 1 >= 0 && board[a][b - 1] != -1) 
            board[a][b - 1] += 1;
        if (a + 1 < row && b + 1 < column && board[a + 1][b + 1] != -1) 
            board[a + 1][b + 1] += 1;
        if (a - 1 >= 0 && b - 1 >= 0 && board[a - 1][b - 1] != -1) 
            board[a - 1][b - 1] += 1;
        if (a + 1 < row && b - 1 >= 0 && board[a + 1][b - 1] != -1) 
            board[a + 1][b - 1] += 1;
        if (a - 1 >= 0 && b + 1 < column && board[a - 1][b + 1] != -1) 
            board[a - 1][b + 1] += 1;
    }
}

// 打印当前游戏地图
inline void printmap()
{
    // 打印二维数组
    for (int i = 0; i < board.size(); i++) 
    {
        for (int j = 0; j < board[i].size(); j++)  
        {
            if (appear[i][j] == 1)
                cout << board[i][j] << " ";
            else if(appear[i][j]==0)
                cout << "x"<< " ";
            else
                cout << "f"<< " ";
        }
        cout << endl; // 每行结束后换行
    }
}

// 打印最终游戏地图
void endmap()
{
    // 打印二维数组
    for (int i = 0; i < board.size(); i++) 
    {
        for (int j = 0; j < board[i].size(); j++)  
        {
            cout << board[i][j] << " ";
        }
        cout << endl; // 每行结束后换行
    }
}
// 递归显示空白格子及其周围的格子
void reveal(int row, int col) {
    // 检查边界条件和是否已经显示
    if (row < 0 || row >= board.size() || col < 0 || col >= board[0].size() || appear[row][col] == 1) {
        return;
    }
    // 标记当前格子为已显示
    appear[row][col] = 1;

    // 如果当前格子是空白格子，递归展开周围的格子
    if (board[row][col] == 0) 
    {
        // 遍历周围的8个方向
        for (int i = -1; i <= 1; i++) 
        {
            for (int j = -1; j <= 1; j++) 
            {
                // 避免对当前格子进行递归调用
                if (i == 0 && j == 0) 
                {
                    continue;
                }
                reveal(row + i, col + j);
            }
        }
    }
}
// 检查所有雷格子是否都被标记
bool allMinesMarked() {
    for (int i = 0; i < board.size(); i++) {
        for (int j = 0; j < board[i].size(); j++) {
            if (board[i][j] == -1 && appear[i][j] != 2) {
                return false;
            }
        }
    }
    return true;
}

int main()
{
    cout << "欢迎来到扫雷游戏!程序获取坐标时在行前输入-即可标记该块,否则为尝试清除该块" << endl;
    cout << "当您标记了所有雷时，您自动获胜;当您尝试清除雷时,您自动失败"<<endl;
    unsigned int num;
    int row;
    int column;
    cout<<"请先给出生成地图的参数"<<endl;
    cout << "请输入雷的个数:";
    cin >> num;
    cout << "请输入地图的行数:";
    cin >> row;
    cout << "请输入地图的列数:";
    cin >> column;
    defmap(num, row, column);
    int crow, ccolumn;
    do
    {
        printmap();
        cout<<"请输入要操作的块的行:";
        cin>>crow;
        cout<<"请输入要操作的块的列:";
        cin>>ccolumn;
        if(crow>=0)
        {
            reveal(crow-1, ccolumn-1);
        }
        if(crow<0)
        {
            crow=-crow;
            int r = crow-1;
            int c = ccolumn-1;
            appear[r][c] = 2; // 标记格子
        }
        if(allMinesMarked())
        {
            cout<<"游戏结束!您成功了"<<endl;
            break;
        }
        if(board[crow-1][ccolumn-1] == -1 && appear[crow-1][ccolumn-1] == 1)
        {
            cout<<"游戏结束!您失败了"<<endl;
            break;
        }
    } 
    while(true);
    endmap();
    return 0;
}
