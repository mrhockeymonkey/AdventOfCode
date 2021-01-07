using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace _4
{

    class Program
    {
        public record Passport
        {
            public string byr; //Birth Year
            public string iyr; //Issue Year
            public string eyr; //Expiration Year
            public string hgt; //Height
            public string hcl; //Hair Color
            public string ecl; //Eye Color
            public string pid; //Passport ID
            public string cid; //Country ID

            public bool IsValid => byr != null
                && iyr != null
                && eyr != null
                && hgt != null
                && hcl != null
                && ecl != null
                && pid != null;
            

            public bool BirthYearValid => int.Parse(byr) >= 1920 && int.Parse(byr) <= 2002;
            public bool IssueYearValid => int.Parse(iyr) >= 2010 && int.Parse(iyr) <= 2020;
            public bool ExpirationYearValid => int.Parse(eyr) >= 2020 && int.Parse(eyr) <= 2030;
            public bool HeightValid => TestHeightValid();
            public bool HairColorValid => new Regex("^\\#[0-9a-f]{6}").IsMatch(hcl);
            public bool EyeColorValid => new string[] { "amb", "blu", "brn", "gry", "grn", "hzl", "oth", }.Contains(ecl);
            public bool PassportIdValid => new Regex("^\\d{9}$").IsMatch(pid);

            public bool StrictIsValid => IsValid
                && BirthYearValid
                && IssueYearValid
                && ExpirationYearValid
                && HeightValid
                && HairColorValid
                && EyeColorValid
                && PassportIdValid;
        

            private bool TestHeightValid()
            {
                if (hgt.Length <= 2)
                {
                    return false;
                }

                var value = int.Parse(hgt.Substring(0, hgt.Length - 2));
                var units = hgt.Substring(hgt.Length - 2);

                return units switch
                {
                    "cm" when value >= 150 && value <= 193 => true,
                    "in" when value >= 59 && value <= 76 => true,
                    _ => false,
                };
            }
        }

        private static Passport ConvertToPassport(string passportData)
        {
            Dictionary<string, string> parsed = new Dictionary<string, string>();
            foreach (var data in passportData.Split(" "))
            {
                var d = data.Split(":");
                parsed[d[0]] = d[1];
            }
            return new Passport
            {
                byr = parsed.GetValueOrDefault("byr"),
                iyr = parsed.GetValueOrDefault("iyr"),
                eyr = parsed.GetValueOrDefault("eyr"),
                hgt = parsed.GetValueOrDefault("hgt"),
                hcl = parsed.GetValueOrDefault("hcl"),
                ecl = parsed.GetValueOrDefault("ecl"),
                pid = parsed.GetValueOrDefault("pid"),
                cid = parsed.GetValueOrDefault("cid"),
            };
        }

        private static IEnumerable<string> NormalizePassportData(string rawInput)
        {
            string[] passportData = rawInput.Split("\r\n\r\n");
            foreach (var data in passportData)
            {
                yield return data.Replace("\r\n", " ");
            }
        }
        
        static void Main(string[] args)
        {
            string input = File.ReadAllText("input.txt");
            var passports = NormalizePassportData(input).Select(x => ConvertToPassport(x)).ToList();

            var answer1 = passports.Where(p => p.IsValid).Count();
            Console.WriteLine($"Answer 1: {answer1}"); //242

            var answer2 = passports.Where(p => p.StrictIsValid).Count();
            Console.WriteLine($"Answer 2: {answer2}"); //186
        }
    }
}
