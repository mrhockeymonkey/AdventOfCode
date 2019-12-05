
min = 234208
max = 765869

# loop through each possible password based on range
passwords = [] 
for i in range(min, max+1, 1):
    s = str(i)
    # check always ascending (probably should be cast to ints for this but this is good enough for single digit)
    if s[0] <= s[1] <= s[2] <= s[3] <= s[4] <= s[5]:
        # we want a duplicate to satisfy the "double" condition, test for removal of duplicate in set theory
        if len(set([s[0], s[1], s[2], s[3], s[4], s[5]])) <= 5:
            passwords.append(s)

print("possible passwords: {0}".format(len(passwords)))

# we know all the passwords are valid so only need to remove when there isnt a "single" double
passwords2 = []
for password in passwords:
    # count the digits (cant use set theory for this one)
    counts = {}
    for digit in password:
        c = {digit: counts.get(digit, 0) + 1}
        counts.update(c)
    # it is valid if there is a digit with count 2
    if 2 in counts.values():
        passwords2.append(password)


print("possible passwords2: {0}".format(len(passwords2)))
