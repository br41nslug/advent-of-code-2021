using System;

class AoCDay6
{
    static void Main(string[] args)
    {
        List<int> fish = args[0].Split(',').Select(x => int.Parse(x)).ToList();
        int days = int.Parse(args[1]);
        Console.WriteLine(String.Join(",", fish.Select(f => f.ToString())));
        for (int i = 0; i < days; i++) {
            int size = fish.Count;
            for (int j = 0; j < size; j++) {
                if (fish[j] == 0) {
                    fish[j] = 6;
                    fish.Add(8);
                } else {
                    fish[j]--;
                }
            }
        }
        Console.WriteLine("Number of fish after {0} days: {1}", days, fish.Count);
    }
}