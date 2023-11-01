use std::{default, cmp::Reverse};

pub fn day2A() {
    //Read input
    //Match matches the various possible resultant situations to various code blocks, in this case handling errs is done in Err and output is done in OK
    let input  = match std::fs::read_to_string("src/day2/input.txt"){
        Err(err) => {
            println!("{}",err);
            "".to_string()

    
    },
        Ok(str) => str
    };
    let rounds = input.split("\n");
    let mut totalscore = 0;
    for round in rounds
        {
            let mut score = 0;
            let round_split = round.split(" ").collect::<Vec<_>>();
            
            if round_split[1] == "X"{
                score+=1;
                if round_split[0] == "A"{
                    score+=3;
                }
                else if round_split[0] == "B"{
                    score+=0;
                }
                else if round_split[0] == "C"{
                    score+=6;
                }
            }
            else if round_split[1] == "Y"{
                score+=2;
                if round_split[0] == "A"{
                    score+=6;
                }
                else if round_split[0] == "B"{
                    score+=3;
                }
                else if round_split[0] == "C"{
                    score+=0;
                }
            }else if round_split[1] == "Z"{
                score+=3;
                if round_split[0] == "A"{
                    score+=0;
                }
                else if round_split[0] == "B"{
                    score+=6;
                }
                else if round_split[0] == "C"{
                    score+=3;
                }
            }
            else
            {
                println!("error:{}","No Match");
            }
            totalscore+=score;
        }
        println!("score:{}", totalscore);
}

pub fn day2B() {
    //Read input
    //Match matches the various possible resultant situations to various code blocks, in this case handling errs is done in Err and output is done in OK
    let input  = match std::fs::read_to_string("src/day2/input.txt"){
        Err(err) => {
            println!("{}",err);
            "".to_string()

    
    },
        Ok(str) => str
    };
    let rounds = input.split("\n");
    let mut totalscore = 0;
    for round in rounds
        {
            let mut score = 0;
            let round_split = round.split(" ").collect::<Vec<_>>();
            
            if round_split[1] == "X"{
                score+=0;
                if round_split[0] == "A"{
                    score+=3;
                }
                else if round_split[0] == "B"{
                    score+=1;
                }
                else if round_split[0] == "C"{
                    score+=2;
                }
            }
            else if round_split[1] == "Y"{
                score+=3;
                if round_split[0] == "A"{
                    score+=1;
                }
                else if round_split[0] == "B"{
                    score+=2;
                }
                else if round_split[0] == "C"{
                    score+=3;
                }
            }else if round_split[1] == "Z"{
                score+=6;
                if round_split[0] == "A"{
                    score+=2;
                }
                else if round_split[0] == "B"{
                    score+=3;
                }
                else if round_split[0] == "C"{
                    score+=1;
                }
            }
            else
            {
                println!("error:{}","No Match");
            }
            totalscore+=score;
        }
        println!("score:{}", totalscore);
}