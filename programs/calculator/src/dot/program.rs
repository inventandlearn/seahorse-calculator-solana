#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Calculator {
    pub owner: Pubkey,
    pub display: i64,
}

impl<'info, 'entrypoint> Calculator {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedCalculator<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let display = account.display;

        Mutable::new(LoadedCalculator {
            __account__: account,
            __programs__: programs_map,
            owner,
            display,
        })
    }

    pub fn store(loaded: Mutable<LoadedCalculator>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let display = loaded.display;

        loaded.__account__.display = display;
    }
}

#[derive(Debug)]
pub struct LoadedCalculator<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Calculator>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub display: i64,
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::ADD
    }
}

pub fn do_operation_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Mutable<LoadedCalculator<'info, '_>>,
    mut op: Operation,
    mut num: i64,
) -> () {
    if !(owner.key() == calculator.borrow().owner) {
        panic!("This is not your calculator!");
    }

    if op == Operation::ADD {
        assign!(
            calculator.borrow_mut().display,
            calculator.borrow().display + num
        );
    } else {
        if op == Operation::SUB {
            assign!(
                calculator.borrow_mut().display,
                calculator.borrow().display - num
            );
        } else {
            if op == Operation::MUL {
                assign!(
                    calculator.borrow_mut().display,
                    calculator.borrow().display * num
                );
            } else {
                if op == Operation::DIV {
                    assign!(
                        calculator.borrow_mut().display,
                        calculator.borrow().display / num
                    );
                }
            }
        }
    }

    solana_program::msg!("{}", calculator.borrow().display);
}

pub fn init_calculator_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Empty<Mutable<LoadedCalculator<'info, '_>>>,
) -> () {
    let mut calculator = calculator.account.clone();

    assign!(calculator.borrow_mut().owner, owner.key());
}

pub fn reset_calculator_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Mutable<LoadedCalculator<'info, '_>>,
) -> () {
    solana_program::msg!(
        "{:?} {} {:?}",
        owner.key(),
        "is resetting".to_string(),
        calculator.borrow().__account__.key()
    );

    if !(owner.key() == calculator.borrow().owner) {
        panic!("This is not your calculator!");
    }

    assign!(calculator.borrow_mut().display, 0);
}
