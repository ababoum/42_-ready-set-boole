/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ast.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mababou <mababou@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/15 13:38:12 by mababou           #+#    #+#             */
/*   Updated: 2022/11/15 15:10:57 by mababou          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::process::id;

enum Op<T> {
	And,
	Or,
	No,
	Xor,
	Then,
	Eq,
	Id(T)
}


struct ASTNode<Bool> {
	left: Option<Box<ASTNode<Bool>>>,
	right: Option<Box<ASTNode<Bool>>>,
	op: Op<Bool>
}

pub fn parse(formula: &str) -> ASTNode<bool>
{
	// init an Abstract Syntax Tree (AST)
	let node: ASTNode<bool>;

	// loop over the formula
	for c in formula.chars() { 
		match c {
			'0' => println!("Zero"),
			'1' => println!("One"),
			'!' => println!("No"),
			'&' => println!("And"),
			'|' => println!("Or"),
			'^' => println!("Xor"),
			'>' => println!("Then"),
			'=' => println!("Eq"),
			_ => println!("Syntax Error")
		}
	}

	return node;
}