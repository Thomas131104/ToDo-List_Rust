use std::collections::BTreeMap;
use log::{info, error};
use crate::work::Work;

pub struct List<'a>
{
    list : Vec<Work<'a>>,
    type_of_task : BTreeMap<String, i32>,
}

impl<'a> List<'a>
{
    // Tạo danh sách mới
    pub fn create() -> Self
    {
        List {
            list : vec![],
            type_of_task : BTreeMap::new(),
        }
    }

    // Thêm công việc mới
    pub fn add(&mut self, task : &'a str)
    {
        if task != ""
        {
            let id = self.list.len() + 1; // Sinh ID dựa trên số lượng công việc hiện tại
            let work = Work::new(id, task); 
            self.list.push(work);

            let task_upper = task.to_uppercase(); 
            *self.type_of_task.entry(task_upper.clone()).or_insert(0) += 1;
            info!("Đã thêm công việc");
        }
        else
        {
            error!("Công việc không thể rỗng");
        }
    }

    // Đánh dấu hoàn thành
    pub fn complete(&mut self, id : usize)
    {
        if let Some(work) = self.list.iter_mut().find(|w| w.get_id() == id && !w.is_completed()) 
        {
            work.mark_complete();
            info!("Đã cập nhật tiến trình");
        }
        else
        {
            error!("Công việc không tồn tại hoặc đã hoàn thành");
        }
    }

    // Xóa công việc
    pub fn remove(&mut self, id: usize)
    {
        if let Some(pos) = self.list.iter().position(|work| work.get_id() == id) 
        {
            let task_upper = self.list[pos].get_name().to_uppercase();
            self.list.remove(pos);

            if let Some(count) = self.type_of_task.get_mut(&task_upper)
            {
                if *count == 1 
                {
                    self.type_of_task.remove(&task_upper);
                } 
                else 
                {
                    *count -= 1;
                }
            }
            info!("Đã xóa công việc");
        } 
        else 
        {
            error!("Công việc không tồn tại");
        }
    }

    // Hiển thị danh sách công việc
    pub fn show(&self)
    {
        println!("DS công việc: ");
        for work in &self.list
        {
            println!(" - {} (ID: {})", work.get_name(), work.get_id());
        }
    }

    // Hiển thị loại công việc và số lượng
    pub fn show_different_task(&self)
    {
        println!("DS các loại công việc và tần suất: ");
        for (key, value) in &self.type_of_task
        {
            println!(" - Công việc: {}  | Số lượng: {}", key, value);
        }
    }
}
