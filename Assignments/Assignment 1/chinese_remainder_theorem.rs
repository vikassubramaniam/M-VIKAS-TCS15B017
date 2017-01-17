
fn mul_inv(mut a: i32,mut b: i32)-> i32
 { let b0=b;let mut t;let mut q;
   let mut x0=0;let mut x1=1;
   if b==1
   {return 1;
   }
   while a>1
   { q=a/b;
     t=b;
     b=a%b;
     a=t;
     t=x0;
     x0=x1-q*x0;
     x1=t;
   }
   if x1<0
   {x1+=b0;
   }
   x1
 }

fn chinese_remainder(n: &mut[i32],a: &mut[i32],len: usize)->i32
 {
   let mut p=0;let mut prod=1;let mut sum=0;
   for i in 0..len
    { prod*=n[i];
    }
   for i in 0..len
    { p=prod/n[i];
      sum += a[i]*mul_inv(p, n[i])*p;
    }
   sum%prod
 }


fn main() {

   let mut n = [3,5,7];
 let mut a = [2,3,2];
 let s = a.len();
   println!("{}",chinese_remainder(&mut n,&mut a,s));
