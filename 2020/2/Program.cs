using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _2
{
    class Program
    {
        private static bool TestPasswordCountPolicy(int min, int max, char letter,  string password )
        {
            var charCount = password.Where(l => l == letter).ToList().Count;
            return charCount >= min && charCount <= max 
                ? true 
                : false;
        }

        private static bool TestPasswordPositionPolicy(int pos1, int pos2, char letter, string password)
        {
            var char1Valid = password[pos1 - 1] == letter;
            var char2Valid = password[pos2 - 1] == letter;
            return char1Valid ^ char2Valid
                ? true
                : false;
        }

        private static int CountValidPasswords(string[] input, Func<int,int,char,string,bool> testMethod)
        {
            var validPasswords = 0;
            foreach (var item in input)
            {
                var parts = item.Split(":");
                var policyParts = parts[0].Split(" ");
                var policyValues = policyParts[0].Split("-");
                var valid = testMethod(
                    int.Parse(policyValues[0]),
                    int.Parse(policyValues[1]),
                    char.Parse(policyParts[1]),
                    parts[1].Trim());

                if (valid)
                {
                    validPasswords++;
                }
            }
            return validPasswords;
        }

        static void Main(string[] args)
        {
            var input = File.ReadAllLines("input.txt");
            
            var answer1 = CountValidPasswords(input, TestPasswordCountPolicy);
            Console.WriteLine($"Answer 1: {answer1}");

            var answer2 = CountValidPasswords(input, TestPasswordPositionPolicy);
            Console.WriteLine($"Answer 2: {answer2}");
        }
    }
}
