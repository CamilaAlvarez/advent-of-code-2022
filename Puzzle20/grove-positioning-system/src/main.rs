// Part1:
// Idea 1:
// The algorithm goes as follows
// 1. Load the numbers in the file into a double linked list A
// 2. Create an array B with the same length as A
// 3. Store the elements of A in B in the same order, storing the index of 0 in V
// 4. For every number in array B
//  4.1. if the number is negative, update the previous pointer of the number
//  4.2. if the number is positive, update the next pointer of the number
// 5. Use V to obtain 0
// 6. Compute A.length() % 1000
// 7. Find the value located after calling next A.length() % 1000 times from 0, Store the value in V1
// 8. Repeat the same for 2000 and 3000, and obtain variables V2 and V3
// 9. Add up V1 + V2 + V3
//
// Idea 2:
// The algorithm goes as follows
// 1. Load the numbers in the file into an array A
//   1.1. Store the positon of the 0 in a variable zero_pos
// 2. Create an array B with the same length as A, fill it with indexes 0..A.length(). This array will indicate which original index is at a specific index
// 2. Create an array C with the same length as A, , fill it with indexes 0..A.length(). This arrat shows the current index of an original index
// 3. For index i in 0..A.length()
//  3.1. Obtain the current index of item i by obtaining current_index = C[i]
//  3.2. value = A[i]
//  3.3. new_position = (current_index + value + A.length()) % + A.length()
//  3.4. if value < 0: new_position = (current_index - 1 + A.length()) % + A.length()
//  3.5. will_move_items_left = current_index <= new_position
//  3.6. Obtain range R that are affected by moving item i:
//      3.6.1. if will_move_items_left, range R is 1..=(new_position - current_index)
//      3.6.2. else, range R is ((new_position - current_index)..0).rev()
//  3.7. For j in R:
//      3.7.1 affected_index = (current_index + j + A.length()) % A.length()
//      3.7.2. original_affected_index = B[affected_index]
//      3.7.3 if will_move_items_left: C[original_affected_index] = (affected_index - 1 + A.length()), B[(affected_index - 1 + A.length())] = original_affected_index
//      3.8.4 else: C[original_affected_index] = (affected_index + 1 + A.length()), B[(affected_index + 1 + A.length())] = original_affected_index
//  3.8. B[new_position] = i // Update the item in the new position
//  3.9. C[i] = new_position // Update the position of the item
// 4. Use V to obtain 0
// 5. Compute A.length() % 1000
// 6. Find the value located after calling next A.length() % 1000 times from 0, Store the value in V1
// 7. Repeat the same for 2000 and 3000, and obtain variables V2 and V3
// 8. Add up V1 + V2 + V3

fn main() {
    println!("Hello, world!");
}
