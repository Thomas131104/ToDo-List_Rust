pub struct Work<'a>
{
    id: usize,         // ID duy nhất cho mỗi công việc
    name: &'a str,     // Tên công việc
    complete: bool,    // Trạng thái hoàn thành
}

impl<'a> Work<'a>
{
    // Tạo công việc mới
    pub fn new(id: usize, name: &'a str) -> Self
    {
        Work {
            id, 
            name,
            complete: false,
        }
    }

    // Đánh dấu công việc là hoàn thành
    pub fn mark_complete(&mut self)
    {
        self.complete = true;
    }

    // Kiểm tra trạng thái hoàn thành
    pub fn is_completed(&self) -> bool
    {
        self.complete
    }

    // Lấy tên công việc
    pub fn get_name(&self) -> &str
    {
        self.name
    }

    // Lấy ID của công việc
    pub fn get_id(&self) -> usize
    {
        self.id
    }
}
