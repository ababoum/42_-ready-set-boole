/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 10:59:57 by mababou           #+#    #+#             */
/*   Updated: 2022/11/14 14:24:01 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// Complexity: O(1) because we iterate over 32 bits max
fn adder(a: u32, b: u32) -> u32 {
	let mut carry: u32 = a & b;
	let sum: u32 = a ^ b;

	if carry != 0
	{
		carry <<= 1;
		return adder(carry, sum);
	}
	return sum;
}

// Complexity: O(1) because we iterate over 32 bits max (over arg b)
fn multiplier(a: u32, b: u32) -> u32 {
	let mut res = 0;
	let mut mult = b;

	let mut index: u32 = 0;
	while mult != 0
	{
		if mult & 1 == 1
		{
			res = adder(res, a << index);
		}
		mult >>= 1;
		index += 1;
	}

	return res;
}

fn main() {
	println!("(0 * 0) = {}", multiplier(0, 0));
    println!("(5 * 0) = {}", multiplier(5, 0));
	println!("(0 * 5) = {}", multiplier(0, 5));
	println!("(2 * 16) = {}", multiplier(2, 16));
	println!("(5 * 5) = {}", multiplier(5, 5));
	println!("(5 * 10) = {}", multiplier(5, 10));
	println!("(1000 * 1234) = {}", multiplier(1000, 1234));
}
