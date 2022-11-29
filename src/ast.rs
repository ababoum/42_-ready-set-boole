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

use core::panic;

pub enum Op<T> {
    And,
    Or,
    No,
    Xor,
    Then,
    Eq,
    Id(T),
}

type ChildNode<T> = Option<Box<ASTNode<T>>>;

pub struct ASTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl ASTNode<bool> {
    pub fn new(op: Op<bool>, l: ASTNode<bool>, r: ASTNode<bool>) -> Self {
        ASTNode::<bool> {
            op: op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

fn and_node(l: ASTNode<bool>, r: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode::new(Op::And, l, r)
}

fn or_node(l: ASTNode<bool>, r: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode::new(Op::Or, l, r)
}

fn xor_node(l: ASTNode<bool>, r: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode::new(Op::Xor, l, r)
}

fn then_node(l: ASTNode<bool>, r: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode::new(Op::Then, l, r)
}

fn eq_node(l: ASTNode<bool>, r: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode::new(Op::Eq, l, r)
}

fn no_node(l: ASTNode<bool>) -> ASTNode<bool> {
    ASTNode {
        op: Op::No,
        left: Some(Box::new(l)),
        right: None,
    }
}

fn val_node(value: bool) -> ASTNode<bool> {
    ASTNode {
        op: Op::Id(value),
        left: None,
        right: None,
    }
}

pub struct ASTTree<T> {
    head: Option<ASTNode<T>>,
}

impl ASTTree<bool> {
    pub fn new(head: ASTNode<bool>) -> Self {
        ASTTree::<bool> { head: Some(head) }
    }

    pub fn collapse(node: &Box<ASTNode<bool>>) -> bool {
        let mut r: Option<bool> = None;
        let mut l: Option<bool> = None;

        if let Some(left) = &node.left {
            l = Some(ASTTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(ASTTree::collapse(right));
        }

        let r = if let Some(x) = r { x } else { false };
        let l = if let Some(x) = l { x } else { false };

        match node.op {
            Op::And => l & r,
            Op::Or => l | r,
            Op::No => !l,
            Op::Xor => l ^ r,
            Op::Then => !l | r,
            Op::Eq => (!l | r) & (!r | l),
            Op::Id(x) => x,
        }
    }
}

pub fn parse(formula: &str) -> ASTTree<bool> {
    let mut reading_stack = Vec::new();

    // loop over the formula
    for c in formula.chars() {
        match c {
            '0' => {
                reading_stack.push(val_node(false));
            }
            '1' => {
                reading_stack.push(val_node(true));
            }
            '!' => {
                if reading_stack.len() == 0 {
                    panic!("Negation operator with insufficient operands")
                } else {
                    let node = no_node(reading_stack.remove(reading_stack.len() - 1));
                    reading_stack.push(node);
                }
            }
            '&' => {
                if reading_stack.len() < 2 {
                    panic!("And operator with insufficient operands")
                } else {
                    let left_operand = reading_stack.remove(reading_stack.len() - 2);
                    let right_operand = reading_stack.remove(reading_stack.len() - 1);
                    let node = and_node(left_operand, right_operand);
                    reading_stack.push(node);
                }
            }
            '|' => {
                if reading_stack.len() < 2 {
                    panic!("Or operator with insufficient operands")
                } else {
                    let left_operand = reading_stack.remove(reading_stack.len() - 2);
                    let right_operand = reading_stack.remove(reading_stack.len() - 1);
                    let node = or_node(left_operand, right_operand);
                    reading_stack.push(node);
                }
            }
            '^' => {
                if reading_stack.len() < 2 {
                    panic!("Xor operator with insufficient operands")
                } else {
                    let left_operand = reading_stack.remove(reading_stack.len() - 2);
                    let right_operand = reading_stack.remove(reading_stack.len() - 1);
                    let node = xor_node(left_operand, right_operand);
                    reading_stack.push(node);
                }
            }
            '>' => {
                if reading_stack.len() < 2 {
                    panic!("Material condition operator with insufficient operands")
                } else {
                    let left_operand = reading_stack.remove(reading_stack.len() - 2);
                    let right_operand = reading_stack.remove(reading_stack.len() - 1);
                    let node = then_node(left_operand, right_operand);
                    reading_stack.push(node);
                }
            }
            '=' => {
                if reading_stack.len() < 2 {
                    panic!("Equivalence operator with insufficient operands")
                } else {
                    let left_operand = reading_stack.remove(reading_stack.len() - 2);
                    let right_operand = reading_stack.remove(reading_stack.len() - 1);
                    let node = eq_node(left_operand, right_operand);
                    reading_stack.push(node);
                }
            }
            _ => panic!("Syntax Error"),
        }
    }

    if reading_stack.len() != 1 {
        panic!("Syntax Error");
    }

    let bt = ASTTree::new(reading_stack.remove(0));
    return bt;
}

pub fn eval_formula(formula: &str) -> bool {
    if formula.len() < 1 {
        panic!("Formula cannot be empty");
    }
    let bt = parse(formula);
    return ASTTree::collapse(&Box::new(bt.head.expect("No head initialized.")));
}
