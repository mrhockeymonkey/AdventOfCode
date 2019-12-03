import sys
import time

def run_program(noun, verb):
    program = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,6,27,1,27,5,31,2,6,31,35,1,5,35,39,2,39,9,43,1,43,5,47,1,10,47,51,1,51,6,55,1,55,10,59,1,59,6,63,2,13,63,67,1,9,67,71,2,6,71,75,1,5,75,79,1,9,79,83,2,6,83,87,1,5,87,91,2,6,91,95,2,95,9,99,1,99,6,103,1,103,13,107,2,13,107,111,2,111,10,115,1,115,6,119,1,6,119,123,2,6,123,127,1,127,5,131,2,131,6,135,1,135,2,139,1,139,9,0,99,2,14,0,0]
    program[1] = noun
    program[2] = verb
    c = 0 # cursor

    for p in program:
        opt = program[c]
        input_1_pos = program[c + 1]
        input_2_pos = program[c + 2]
        output_pos = program[c + 3]

        if opt == 1: 
            #add
            program[output_pos] = program[input_1_pos] + program[input_2_pos]
        elif opt == 2: 
            # multiply
            program[output_pos] = program[input_1_pos] * program[input_2_pos]
        elif opt == 99: 
            # halt
            return program[0]
        else:
            raise Exception("Unknown opt code")

        c += 4

if __name__ == '__main__':
    part_1 = run_program(12, 1)
    print("Part 1: {0}".format(part_1))

    start_time = time.process_time()
    for n in range(100):
        for v in range(100):
            result = run_program(n, v)
            # print("noun: {0}, verb: {1}, result: {2}".format(n, v, result))
            if result == 19690720:
                process_time = (time.process_time() - start_time)*100
                print("Part 2: {0}, Time: {1}".format(100 * n + v, process_time))
                sys.exit()