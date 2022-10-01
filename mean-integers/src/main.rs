use std::io;
use factorial::Factorial;
use round::Round;


fn main(){

    //We define the limit of the sum of integers i: from 0 to n and we divide it by n
    //Mean of integers is: n+1/2

    fn mean_of_sum(n: i32) -> i32 {
        return n+1/2;
    }

    //Now try get some data, giving an imput of primes numbers and checking how much their mean_of_sum skews
        //First we create an array of prime numbers

    fn generate_nth_prime(n: i32) -> i32{
        //We're gonna use C.P. Williams 1964 formula to evaluate the n-th prime
        //Formula is: 1 + Sum of i=1..2^n(floor(n/sum of j=1 to i(floor(cosine(pi)*((j-1)!+1)/j)^2))^1/n)
        //We're going to scompone this big formula from inside to not get wrong calcs...
        let mut extern_sum = 0;
        let mut current = 0;

        //1. Extern sum
        for i in 1..2^n{
            current = Round::round_down((n/sum_denominator(i))^1/n);
            extern_sum = extern_sum + current;
        }

        return 1 + extern_sum

    }
    fn sum_denominator(index: i32) -> i32 {        
        let pi = std::f32::consts::PI;  
        let mut intern_sum = 0;
        let mut current = 0;


        for j in 1..index {
            current = Round::round_down(((pi.cos())*(Factorial(j-1)+1)/j)^2);
            intern_sum = intern_sum + current;
        } 

        return intern_sum;
    }




    //let array_of_primes: [i32, 5] = [generate_nth_prime(1), generate_nth_prime(2), generate_nth_prime(3), generate_nth_prime(4), generate_nth_prime(5)];

/*     println!("First 5 primes are: {}", array_of_primes);
    println!("The mean of the sum of the first 5 prime numbers is: {}", mean_of_sum(array_of_primes)); */
/*     fn skews_of_mean(){

    } */

}