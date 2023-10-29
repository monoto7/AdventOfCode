use std::{default, cmp::Reverse};

pub fn day1() {
    //Read input
    //Match matches the various possible resultant situations to various code blocks, in this case handling errs is done in Err and output is done in OK
    let input  = match std::fs::read_to_string("src/day1/input.txt"){
        Err(err) => {
            println!("{}",err);
            "".to_string()

    
    },
        Ok(str) => str
    };
    //Get individual elf backpacks
    let backpacks = input.split("\n\n");
    let mut vectint = Vec::new();
    for backpack in backpacks {
        //break elf backpack down into meals
        let meals = backpack.split("\n");
        let mut sumtotal = 0;
        for meal in meals {
            let int: i32 = match meal.parse() {
                Err(_err) => {
                    println!("error:{}",meal);
                    0
                },
                Ok(val) => val
            };

            sumtotal+=int;


        }
        if sumtotal!=0
        {
            //wrap values in reverse so that the sorting is inverted(descending instead of ascending)
            vectint.push(Reverse(sumtotal));
        }
    }
    //Sort vector
    vectint.sort();


    println!("1st: {:?}",vectint[0].0);
    println!("2nd: {:?}",vectint[1].0);
    println!("3rd: {:?}",vectint[2].0);
    let int_total_top3 = vectint[0].0 + vectint[1].0 + vectint[2].0;
    println!("totalCals Top3: {:?}",int_total_top3);

}
