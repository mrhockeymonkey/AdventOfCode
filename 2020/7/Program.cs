using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text.RegularExpressions;

namespace _7
{
    // Vertex: represents a point in a graph data structure
    class Vertex
    {
        public string Value;
        public List<Edge> Edges = new List<Edge>();

        public Vertex(string value)
        {
            Value = value;
        }

        public void AddEdge(Vertex child, int weight)
        {
            // throw argument exception if duplicate edge?
            Edges.Add(new Edge(this, child, weight));
        }

        public override string ToString()
        {
            return $"Vertex: {Value}";
        }
    }

    // Edge: represents a weighted connection between two points in one direction
    class Edge
    {
        public Vertex Parent;
        public Vertex Child;
        public int Weight;

        public Edge(Vertex parent, Vertex child, int weight)
        {
            Parent = parent;
            Child = child;
            Weight = weight;
        }

        public override string ToString()
        {
            return $"Edge: {Parent}->{Child} ({Weight})";
        }
    }

    class Graph
    {
        public List<Vertex> Vertices = new List<Vertex>();
        public int[,] AdjMatrix;

        public Vertex GetOrCreateVertex(string value)
        {
            var vertex = Vertices.FirstOrDefault(v => v.Value == value);
            if (vertex == null)
            {
                vertex = new Vertex(value);
                Vertices.Add(vertex);
            }
            return vertex;
        }

        public void AddEdge(string parent, string child, int weight)
        {
            Console.Write($"{weight} {child},");

            var parentVertex = GetOrCreateVertex(parent);
            var childVertex = GetOrCreateVertex(child);
            parentVertex.AddEdge(childVertex, weight); 
        }

        public void CreateAdjacencyMatrix()
        {
            // the adjacency matrix is a 2-d array of weighted edges from the parent x, to child y
            // if AdjMatrix[x, y] > 0 there exists an edge between the two vertices (in one direction only)
            AdjMatrix = new int[Vertices.Count, Vertices.Count];
            for (int i = 0; i < Vertices.Count; i++)
            {
                var parentVertex = Vertices[i];
                foreach (var edge in parentVertex.Edges)
                {
                    AdjMatrix[i, Vertices.IndexOf(edge.Child)] = edge.Weight;
                }
            }
        }

        // Finds all parents vertices relative to a target vertex value
        public IEnumerable<Vertex> GetParentVertices(string target)
        {
            var queue = new Queue<int>(); // keep track of verties to check
            var seen = new HashSet<int>(); // keep track of vertices seen
            var targetIndex = Vertices.FindIndex(x => x.Value == target);
            queue.Enqueue(targetIndex);

            // this is a "breadth-first-search" of the graph only following parents to a given vertex
            while (queue.Count > 0)
            {
                var childIndex = queue.Dequeue();
                if (seen.Contains(childIndex))
                {
                    continue;
                }
                seen.Add(childIndex);

                // loop through all parents to see if any have an edge directed at this child
                for (int i = 0; i < Vertices.Count; i++)
                {
                    if (AdjMatrix[i, childIndex] > 0)
                    {
                        queue.Enqueue(i);
                    }
                }
            }

            return seen
                .Where(i => i != targetIndex)
                .Select(i => Vertices[i])
                .ToArray();
        }
    }


    class Program
    {
        // helper method to parse input into a graph
        public static Graph ConvertToGraph(string[] input)
        {
            var graph = new Graph();
            foreach (var line in input)
            {
                var parentChildrenSplit = line.Split("contain");
                var parent = parentChildrenSplit[0].Trim().Replace(" ", "").TrimEnd('s');
                if (parentChildrenSplit[1].Trim() == "no other bags.")
                {
                    Console.WriteLine($"{parent} contains no other bags");
                    continue;
                }
                Console.Write($"{parent} contains ");
                foreach (var childLine in parentChildrenSplit[1].TrimEnd('.').Split(","))
                {
                    var matches = Regex.Match(childLine.Trim(), "^(?<weight>\\d+)\\s(?<bag>\\w+\\s\\w+\\s\\w+)", RegexOptions.IgnoreCase);
                
                    graph.AddEdge(
                        parent,
                        child: matches.Groups["bag"].Value.Replace(" ", "").TrimEnd('s'),
                        weight: int.Parse(matches.Groups["weight"].Value.Trim()));
                }
                Console.Write("\r\n");
            }
            graph.CreateAdjacencyMatrix();
            return graph;
        }

        static void Main(string[] args)
        {
            /*
            The input can be expressed as a "weighted directed graph" with bag color as vertces and number of bag
            that can be contained as the weights. 
            
            There is no garuntee of a "root bag" and any bag may have multiple parents so a tree would not suffice
            */

            var testGraph = ConvertToGraph(File.ReadAllLines("test_input.txt"));
            var testParents = testGraph.GetParentVertices("shinygoldbag");
            var testAnswer1 = testParents.Count();
            Console.WriteLine($"Answer1: {testAnswer1}");
            Console.ReadLine();

            var graph = ConvertToGraph(File.ReadAllLines("input.txt"));
            var parents = graph.GetParentVertices("shinygoldbag");
            var answer1 = parents.Count();
            Console.WriteLine($"Answer1: {answer1}");
        }
    }
}
