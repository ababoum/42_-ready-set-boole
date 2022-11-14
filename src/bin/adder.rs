/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 10:59:57 by mababou           #+#    #+#             */
/*   Updated: 2022/11/14 13:47:07 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

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

fn main() {
	println!("(0 + 0) = {}", adder(0, 0));
    println!("(5 + 0) = {}", adder(5, 0));
	println!("(0 + 5) = {}", adder(0, 5));
	println!("(5 + 5) = {}", adder(5, 5));
	println!("(5 + 10) = {}", adder(5, 10));
	println!("(1000 + 1234) = {}", adder(1000, 1234));
	println!("(2147483647 + 2147483647) = {}", adder(u32::MAX, u32::MAX));
	println!("(2 + 2147483647 + 2147483647) = {} // overflow!", adder(2, adder(u32::MAX, u32::MAX)));
}
