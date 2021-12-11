using System;
using System.Linq;

// class AoCDay6Part1
// {
//     static void Main(string[] args)
//     {
//         List<int> fish = args[0].Split(',').Select(x => int.Parse(x)).ToList();
//         int days = int.Parse(args[1]);
//         Console.WriteLine(String.Join(",", fish.Select(f => f.ToString())));
//         for (int i = 0; i < days; i++) {
//             int size = fish.Count;
//             for (int j = 0; j < size; j++) {
//                 if (fish[j] == 0) {
//                     fish[j] = 6;
//                     fish.Add(8);
//                 } else {
//                     fish[j]--;
//                 }
//             }
//         }
//         Console.WriteLine("Number of fish after {0} days: {1}", days, fish.Count);
//     }
// }

class AoCDay6Part2
{
    static void runDay(ref List<int> cycles, ref List<ulong> amounts)
    {
        int size = cycles.Count;
        for (int i = 0; i < size; i++) {
            if (cycles[i] == 0) {
                cycles[i] = 6;
                cycles.Add(8);
                amounts.Add(amounts[i]);
            } else {
                cycles[i]--;
            }
        }
    }

    static void deduplicate(ref List<int> cycles, ref List<ulong> amounts)
    {
        int size = cycles.Count;
        var _cycles = new List<int> {};
        var _amounts = new List<ulong> {};
        for (int i = 0; i < size; i++) {
            if (_cycles.Contains(cycles[i])) {
                int index = _cycles.IndexOf(cycles[i]);
                _amounts[index] += amounts[i];
            } else {
                _cycles.Add(cycles[i]);
                _amounts.Add(amounts[i]);
            }
        }
        cycles = _cycles;
        amounts = _amounts;
    }

    static ulong calcFishes(List<ulong> amounts) {
        ulong result = 0;
        foreach (ulong amount in amounts) {
            result += amount;
        }
        return result;
    }

    static void Main(string[] args)
    {
        var fcycle = new List<int> {};
        var famount = new List<ulong> {};
        foreach (int f in args[0].Split(',').Select(x => int.Parse(x))) {
            if ( ! fcycle.Contains(f)) {
                fcycle.Add(f);
                famount.Add(0);
            }
            famount[fcycle.IndexOf(f)]++;
        }
        int days = int.Parse(args[1]);
        for (int i = 1; i <= days; i++) {
            runDay(ref fcycle, ref famount);
            deduplicate(ref fcycle, ref famount);
            if (i % 10 == 0) {
                Console.WriteLine("Number of fish after {0} days: {1}", i, calcFishes(famount).ToString());
            }
        }
        Console.WriteLine("\nNumber of fish after {0} days: {1}", days, calcFishes(famount).ToString());
    }
}