/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_bytes.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nomoulin <nomoulin@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/09/27 03:15:54 by nomoulin          #+#    #+#             */
/*   Updated: 2023/09/27 03:48:33 by nomoulin         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn print_bytes(s: &str) {
    for byte in s.bytes() {
        println!("{}", byte);
    }
}

fn main() {
    print_bytes("Salut");
}