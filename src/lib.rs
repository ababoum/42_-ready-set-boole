/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 15:27:45 by mababou           #+#    #+#             */
/*   Updated: 2022/11/15 14:53:17 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod ast;

// Complexity: O(1) because we iterate over 32 bits max
pub fn adder(a: u32, b: u32) -> u32
{
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
pub fn multiplier(a: u32, b: u32) -> u32
{
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

pub fn gray_code(n: u32) -> u32
{
	return n ^ (n >> 1);
}