/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/28 19:15:04 by nstabel       #+#    #+#                 */
/*   Updated: 2021/05/20 20:15:37 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use computor::Computor;
use std::env;

fn main () {
    let mut computor = Computor::default();

    for arg in env::args().skip(1) {
        computor.ingest(arg);
        computor.tokenize();
        computor.print();
    }


}

// https://petermalmgren.com/token-scanning-with-rust/
// 5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0