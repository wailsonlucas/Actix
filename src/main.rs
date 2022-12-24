use yew::prelude::*;

#[function_component]
fn App() -> Html {
  

   html!{ <div class="container">
            <nav>
               <b>{"Yassine Assembly"}</b>
            </nav>
            <header>
               <p>{"Web Assembly app with Rust Language"}</p>
            </header>
            <main>
               <div class="box">
                  <div class="image-container">
                     <img src="rust.png" alt="logo of Rust programming language"/>
                  </div>
                  <div class="content">
                     <b>{"Rust language"}</b>
                     <i>{"built the html using my lovely Rust language"}</i>
                  </div>
               </div>
                <div class="box">
                  <div class="image-container">
                     <img src="wasm.png" alt="logo of Rust programming language"/>
                  </div>
                  <div class="content">
                     <b>{"Web Assembly"}</b>
                     <i>{"turned it into binaries using WASM"}</i>
                  </div>
               </div>
                <div class="box">
                  <div class="image-container">
                     <img src="web.png" alt="logo of Rust programming language"/>
                  </div>
                  <div class="content">
                     <b>{"Web"}</b>
                     <i>{"now i am live on web"}</i>
                  </div>
               </div>
            </main>
            <section class="input-section">
               <div>
                  <input />
                  <button>{"check"}</button>
               </div>
            </section>
         </div>
      }
}

fn main() {
   yew::Renderer::<App>::new().render();   














   // let mut nums:Vec<i32> = vec![14,21,54,41,41,23, 11];

   //Finding the mean of nums
   // let len = nums.len();
   // let sum = nums.iter().sum::<i32>();
   // let mean = sum / len as i32;
   // println!("{:?}", mean);   

   //Finding the median of sorted nums
   // nums.sort();
   // println!("{:?}", nums);
   // if len % 2 == 0 {
   //    let first_index = (len/2)-1;
   //    let last_index = first_index+1;
   //    let median = (nums[first_index]+nums[last_index]) / 2;
   //    println!("{}", median);
   // } else {
   //    let index = (len - 1)/2;
   //    let median = nums[index];
   //    println!("{}", median);
   // }


   //Finding the item that occurs more often
   // let mut occure_table:HashMap<i32, i32> = HashMap::new();

   // let mut i:usize = 0;
   //  while i < len {
   //    occure_table.entry(nums[i]).or_insert(0);
   //    i+=1;
   // }
   // let mut j:usize = 0;
   // while j < len {
   //    let occure_count = occure_table.get(&nums[j]).unwrap();
   //    let updated_occure_count = occure_count + 1;
   //    occure_table.insert(nums[j] ,updated_occure_count);

   //    j+=1;
   // }
   // println!("{:?}", occure_table);



   

}