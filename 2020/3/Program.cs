using System;
using System.IO;

namespace _3
{
    class Program
    {
        private static char tree = '#';

        private static uint TraverseSlopeAndCountTrees(int vectorX, int vectorY, string[] input)
        {
            var treesEncountered = 0u;
            var slopeWidth = input[0].Length;
            var slopeLength = input.Length;
            (int posX, int posY) pos = (0, 0);

            while (pos.posY < slopeLength -1)
            {
                pos.posX = (pos.posX + vectorX) % slopeWidth;
                pos.posY += vectorY;

                if (input[pos.posY][pos.posX] == tree)
                {
                    //Console.WriteLine($"Encountered tree at {pos}");
                    treesEncountered++;
                }
            }
            return treesEncountered;
        }

        static void Main(string[] args)
        {
            var input = File.ReadAllLines("input.txt");
            var answer1 = TraverseSlopeAndCountTrees(3, 1, input);
            Console.WriteLine($"Answer 1: {answer1}");

            var slope11 = TraverseSlopeAndCountTrees(1, 1, input);
            var slope31 = TraverseSlopeAndCountTrees(3, 1, input);
            var slope51 = TraverseSlopeAndCountTrees(5, 1, input);
            var slope71 = TraverseSlopeAndCountTrees(7, 1, input);
            var slope12 = TraverseSlopeAndCountTrees(1, 2, input);
            var answer2 = slope11 * slope31 * slope51 * slope71 * slope12;
            Console.WriteLine($"Answer 2: {answer2}");

        }
    }
}
