using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _6
{
    class Program
    {
        class GroupAnswers
        {
            public List<HashSet<char>> AnswerSets = new List<HashSet<char>>();
            private HashSet<char> _potentialAnswers = new HashSet<char> { 
                'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'};

            public int UniqueAnswerCount
            {
                get
                {
                    var combined = new HashSet<char>();
                    foreach (var answerSet in AnswerSets)
                    {
                        combined.UnionWith(answerSet);
                    }
                    return combined.Count;
                }
            }

            public int JointAnswersCount
            {
                get
                {
                    var intersection = _potentialAnswers;
                    foreach (var answerSet in AnswerSets)
                    {
                        intersection.IntersectWith(answerSet);
                    }
                    return intersection.Count;
                }
            }
        }

        private static Dictionary<int, GroupAnswers> ParseInput(string[] input)
        {
            var parsed = new Dictionary<int, GroupAnswers>();
            var i = 0;
            
            foreach (var line in input)
            {
                if (String.IsNullOrWhiteSpace(line))
                {
                    i++;
                    continue;
                }
                
                if (!parsed.ContainsKey(i))
                {
                    parsed.Add(i, new GroupAnswers());
                }

                var lineSet = new HashSet<char>(); 
                foreach (var character in line)
                {
                    lineSet.Add(character);
                }
                parsed[i].AnswerSets.Add(lineSet);
            }
            return parsed;
        }

        static void Main(string[] args)
        {
            var input = File.ReadAllLines("input.txt");
            var parsed = ParseInput(input);

            var answer1 = parsed
                .Select(group => group.Value.UniqueAnswerCount)
                .Sum();
            Console.WriteLine($"Answer 1: {answer1}"); // 6291

            var answer2 = parsed
                .Select(group => group.Value.JointAnswersCount)
                .Sum();
            Console.WriteLine($"Answer 2: {answer2}"); // 
        }
    }
}
