/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   gray.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 15:39:50 by mababou           #+#    #+#             */
/*   Updated: 2022/11/14 15:42:13 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rsb::gray_code;

fn main() {
	println!("0 => {}", gray_code(0));
	println!("2 => {}", gray_code(2));
	println!("3 => {}", gray_code(3));
	println!("4 => {}", gray_code(4));
	println!("5 => {}", gray_code(5));
	println!("6 => {}", gray_code(6));
	println!("7 => {}", gray_code(7));
	println!("8 => {}", gray_code(8));
}
