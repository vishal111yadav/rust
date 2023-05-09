fn main(){
    // let a=10;
    // let b=20;
    // println!("Hello world!, {} {}", a, b);
//  //unsigned integer=>u8,u16,u32,u68,128
//     let unsigned: u8=10;
// //signed integer i8,i16,i32,i64,i128
//     let signed: i8=-10;
// //float
//     let float:f32=1.2;

//     println!("unsigned:{} sign{} float{}",unsigned,signed,float);
// //char -can only be
// let letter="c";
// let emoji="\u{1f600}";

// println!("letter:{} emoji{} ",letter,emoji);

// let is_true:bool =true;

// println!("is _true:{}",is_true);
// let arr:[u8;3]=[1,2,3];
// let other_arr:[u8;5]=[100;5];

// println!("index:{},length:{}",arr[0],other_arr.len());

// //print structure of array and other object
// println!("{:?}",other_arr);

  //variable:tuple
// let tuple:(u8,bool,f32)=(5,true,2.1);
// let tuple2=(3,5);
//    //print structure of array and other object
// println!("first: {},second: {},third: {}",tuple.0,tuple.1,tuple.2);
// println!("{:?}",tuple2);
// let (a,b,c)=tuple;
// //destructuring
// println!("first: {},second: {},third: {}",a,b,c);

// //function calling
// println!("{}",is_even(3));
// //mutability-->means changing the value
// let mut num=5;
// num=3;
// println!("{}",num);

// //array+slices
// let arr =[0,1,2,3];//array we know the length ,for slice we dont know the length
// let slice =&arr[1..3];//first value is inclusive and last is exclusive[1,2],&is for reference
// borrowing_slice(arr,slice);

// //string

// let str: &str="hello world";//$str slice of string
// let mut string:String= String::from("Hello world");//this create a string object

// let slice=&string[.. 6];//[.. 6]get everything from index 0 to 6 not includeing index 6
// slice.len();

// string.push('1');//it allows you to add an chaar
// string.push_str("! bob");
// string =string.replace("Hello","Bye");
// println!("{}",string);

// //If else statement
// let n=3;
// if n > 0 {
//     println!("greater than 0");
// }
// else if n < 0{
//     println!("less than 0");
// }
// else{
//     println!("is 0");
// }

// //for loop
// for i in 0..6{//its always exclude the last value
//     println!("{}",i);
// }
// //while loop
// let mut i=0;
// while i < 4 {
//     println!("{}",i);
//     i +=1;
//     if i ==3 {
//         println!("exit");
//         break//continue  //in break or coninue dont use semicolon 
//     }

// }
    //     //match statement
    //  let i = 4;
    //  match i {
    //     0 => println!("0"),
    //     1 | 2=> println!("1,2"),
    //     3..=4 => println!("3,4"),//4 is inclusive because of ..=
    //     _=> println!("default")
    //  }   
    
    //struct
    let name=String::from("Bird");
        let bird=Bird{name:name,attack:5};
        bird.print_name();
 

}
// //function
// pub fn is_even(num:u8)->bool{//pub means public
//     let digit : u8 =num %2;
//     digit == 0//with out semicolon return
// }

// fn borrowing_slice(arr:[u8;4],slice:&[u8]){
//     println!("{:?}",arr);
//     println!("{:?}",slice);
//     println!("length:{}",slice.len());
//     println!("{} {}",slice[0],slice[1]);
// }

//     //struct
struct Bird{
name: String ,
attack: u64 
}

impl Bird{ //impl is for implement
    fn print_name(&self){
        println!("{} {}",self.name,self.attack);
    }
}

trait Animal{//trait is an interface
    fn car_fly(&self)->bool;
    fn is_animal(&self)->bool{
        true
    }
}