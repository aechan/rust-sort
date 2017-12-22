fn main() {
    do_bubble();
    do_insertion();
}

fn do_bubble() {
    let mut sd = SortData::new(vec![ 10, 33, 1, 888, 23 ]);
    println!("======================================");
    println!("Bubble Sort");
    
    println!("Sorting: {:?}", sd.data);
    sd.bubble_sort();
    println!("Sorted: {:?}", sd.data);
    println!("======================================");

}

fn do_insertion() {
    let mut sd = SortData::new(vec![ 10, 33, 1, 888, 23 ]);
    println!("======================================");
    println!("Insertion Sort");
    println!("Sorting: {:?}", sd.data);
    sd.insertion_sort();
    println!("Sorted: {:?}", sd.data);
    println!("======================================");
}

struct SortData {
    data: Vec<i32>
}

impl SortData {
    fn new(data: Vec<i32>) -> SortData {
        SortData { data: data }
    }

    fn insertion_sort(&mut self) {
        let input = &mut self.data;
        let n = input.len();

        for i in 1..n as i32 {
            let key = input[i as usize];
            let mut j = i-1;

            while j>=0 && input[j as usize] > key {
                input[(j+1) as usize] = input[j as usize];
                j=j-1;
            }
            input[(j+1) as usize] = key;
        }
    }

    fn bubble_sort(&mut self) {
        let input = &mut self.data;
        let n = input.len();

        for i in 0..n {
            for j in 0..n-i-1 {
                if input[j] > input[j+1] {
                    let temp = input[j];
                    input[j] = input[j+1];
                    input[j+1] = temp;
                }
            }
        }
    }

}