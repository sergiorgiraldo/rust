use primes::factors;
use std::collections::HashMap;

pub fn do_it() {
    let res;
    let mut n = 1;
    loop {
        let sum = n * (n + 1) / 2;
        let mut divisors = 1;
        // Usando este metodo pra calcular os divisores
        //
        // 1.Realizar a decomposição em fatores primos
        let factors = factors(sum);
        // Achar os expoentes de cada fator
        let mut map: HashMap<u64, usize> = HashMap::new();
        for factor in factors {
            *map.entry(factor).or_default() += 1;
        }
        // 2.Adicionar uma unidade ao expoente de cada fator
        // 3.Multiplicar os resultados obtidos no passo anterior.
        for (_, value) in map {
            divisors *= value + 1;
        }

        if divisors > 500 {
            res = sum;
            break;
        } else {
            n += 1;
        }
    }
    println!("p012=>{}", res);
}
