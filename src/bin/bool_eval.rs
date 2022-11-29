/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bool_eval.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 15:56:38 by mababou           #+#    #+#             */
/*   Updated: 2022/11/15 14:59:15 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rsb::ast::eval_formula;

fn main() {
    println!("10& => {}", eval_formula("10&"));
	println!("10| => {}", eval_formula("10|"));
	println!("10|1& => {}", eval_formula("10|1&"));
	println!("101|& => {}", eval_formula("101|&"));
	println!("11> => {}", eval_formula("11>"));
	println!("10= => {}", eval_formula("10="));
	println!("1011||= => {}", eval_formula("1011||="));
}
