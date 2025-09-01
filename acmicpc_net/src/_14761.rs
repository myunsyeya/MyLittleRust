/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   _14761.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: wonjjeon <yosouls01@gmail.com>             +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/09/01 11:26:49 by wonjjeon          #+#    #+#             */
/*   Updated: 2025/09/01 11:27:03 by wonjjeon         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::str::FromStr;

#[test]
fn test() {
    if let [x, y, n] = read_vec::<i8>().as_slice() {
        for i in 1..n + 1 {
            match (i % x, i % y) {
                (0, 0) => println!("FizzBuzz"),
                (0, _) => println!("Fizz"),
                (_, 0) => println!("Buzz"),
                _ => println!("{}", i),
            }
        }
    }
}

#[cfg(test)]
fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim_end().to_string()
}

#[cfg(test)]
fn read_vec<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let line = read_line();
    line.split_whitespace()
        .map(|x| T::from_str(x).expect("failed to parse value"))
        .collect()
}
