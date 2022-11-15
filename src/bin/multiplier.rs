/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiplier.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 10:59:57 by mababou           #+#    #+#             */
/*   Updated: 2022/11/14 15:39:46 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rsb::multiplier;

fn main() {
	println!("(0 * 0) = {}", multiplier(0, 0));
    println!("(5 * 0) = {}", multiplier(5, 0));
	println!("(0 * 5) = {}", multiplier(0, 5));
	println!("(2 * 16) = {}", multiplier(2, 16));
	println!("(5 * 5) = {}", multiplier(5, 5));
	println!("(5 * 10) = {}", multiplier(5, 10));
	println!("(1000 * 1234) = {}", multiplier(1000, 1234));
}
