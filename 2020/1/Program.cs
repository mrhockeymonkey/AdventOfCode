using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _1
{
    class Program
    {
        private static List<int> _expenseAccountData = new List<int>();
        private static int _total = 2020;

        static void LoadInput()
        {
            var lines = File.ReadAllLines("input.txt");
            foreach (var line in lines)
            {
                _expenseAccountData.Add(int.Parse(line));
            }
        }

        static int FindBySubracting()
        {
            int answer = 0;
            foreach (var expense in _expenseAccountData)
            {
                int difference = _total - expense;

                if (_expenseAccountData.Contains(difference))
                {
                    Console.WriteLine($"{expense} x {difference}");
                    answer = expense * difference;
                    break;
                }
            }
            return answer;
        }

        static int FindByLooping()
        {
            var answer = 0;
            for (int i=0; i < _expenseAccountData.Count; i++)
            {
                for (int j = 0; j < _expenseAccountData.Count; j++)
                {
                    if (i == j) { continue; }
                    for (int k = 0; k < _expenseAccountData.Count; k++)
                    {
                        if (j == k) { continue; }
                        if (_expenseAccountData[i] + _expenseAccountData[j] + _expenseAccountData[k] == _total )
                        {
                            answer = _expenseAccountData[i] * _expenseAccountData[j] * _expenseAccountData[k];
                            break;
                        }
                    }
                }
            }
            return answer;
        }

        static void Main(string[] args)
        {
            LoadInput();
            var answer1 = FindBySubracting();
            Console.WriteLine($"Answer 1: {answer1}");

            var answer2 = FindByLooping();
            Console.WriteLine($"Answer 2: {answer2}");
        }
    }
}
