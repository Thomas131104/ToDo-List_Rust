mod work;
mod list;
use crate::list::List;

fn main() {
    let mut my_list = List::create();

    // Thêm công việc
    my_list.add("Học Rust");
    my_list.add("Làm bài tập");
    my_list.add("Học Rust");


    
    // Hiển thị danh sách
    my_list.show();
    // DS công việc: 
    // - Học Rust (ID: 1)
    // - Làm bài tập (ID: 2)
    // - Học Rust (ID: 3)



    my_list.show_different_task();
    // DS các loại công việc và tần suất:      
    // - Công việc: HỌC RUST  | Số lượng: 2   
    // - Công việc: LÀM BÀI TẬP  | Số lượng: 1



    // Hoàn thành công việc thứ 1
    my_list.complete(1);
    my_list.show();
    // DS công việc:
    // - Học Rust (ID: 1)
    // - Làm bài tập (ID: 2)
    // - Học Rust (ID: 3)



    // Xóa công việc thứ 2
    my_list.remove(2);
    my_list.show();
    // DS công việc:
    // - Học Rust (ID: 1)
    // - Học Rust (ID: 3)
}
