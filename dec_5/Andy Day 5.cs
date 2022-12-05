// See https://aka.ms/new-console-template for more information
using System.Data;
using System.Data.OleDb;

Console.WriteLine("Hello, World!");
IDictionary<int, List<char>> boxes = new Dictionary<int, List<char>>();
var list1 = new List<char>() {'R','P','C','D','B','G'};
var list2 = new List<char>() {'H','V','G'};
var list3 = new List<char>() {'N','S','Q','D','J','P','M'};
var list4 = new List<char>() {'P','S','L','G','D','C','N','M'};
var list5 = new List<char>() {'J','B','N','C','P','F','L','S'};
var list6 = new List<char>() {'Q','B','D','Z','V','G','T','S'};
var list7 = new List<char>() {'B','Z','M','H','F','T','Q'};
var list8 = new List<char>() {'C','M','D','B','F'};
var list9 = new List<char>() {'F','C','Q','G'};
boxes.Add(1, list1);
boxes.Add(2, list2);
boxes.Add(3, list3);
boxes.Add(4, list4);
boxes.Add(5, list5);
boxes.Add(6, list6);
boxes.Add(7, list7);
boxes.Add(8, list8);
boxes.Add(9, list9);

//List<List<int>> moves = new List<int, List<int>>();
using(var reader = new StreamReader(@"C:\Users\249916\OneDrive - CarMax\Desktop\Advent Of Code\day 5 part 1 moves.csv"))
{
    int i = 0;    
    while (!reader.EndOfStream)
    {
        i++;
        var line = reader.ReadLine();
        if(i == 1) { continue;}       
        
        var values = line.Split(',');
        //Console.WriteLine(values[0]);
        int numToMove = Convert.ToInt32(values[1]);
        int moveFrom = Convert.ToInt32(values[2]);
        int moveTo = Convert.ToInt32(values[3]);
        //Console.WriteLine("from");
        //boxes[moveFrom].ForEach(p => Console.WriteLine(p));
        
        var boxToMove = boxes[moveFrom].TakeLast(numToMove).ToList();
        var currentCnt = boxes[moveFrom].Count();
        for(int j = currentCnt- 1; j > currentCnt - 1 - boxToMove.Count(); j-- ) 
        {
            boxes[moveFrom].RemoveAt(j);
        }
        //Console.WriteLine("new from");
        //boxes[moveFrom].ForEach(p => Console.WriteLine(p));

        //Console.WriteLine("to");
        //boxes[moveTo].ForEach(p => Console.WriteLine(p));
        boxes[moveTo].AddRange(boxToMove);
        //Console.WriteLine("new to");
        //boxes[moveTo].ForEach(p => Console.WriteLine(p));
        
    }
    foreach(var thing in boxes)
    {
        Console.WriteLine(thing.Value.LastOrDefault());
    }
}
