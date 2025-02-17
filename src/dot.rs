/*a dot is a data point chowing the completion of a task.
Example: If someone wanted to keep track of how many days they brushed their teeth and how many times
When they mark that they have done this task, a dot will be made. If they needed to brush their teeth twice a day every day
Then total would be set to 2, allowing them to complete their task twice before returning true for is complete*/
use chrono::prelude::*;
pub struct Dot{
    completion_date: NaiveDate,
    complete: bool,
    progress: i32,
}
impl Dot{
    //Create a new Dot using current date
    pub fn new(year: i32,month: u32,day: u32) -> Dot{
        Dot { completion_date: NaiveDate::from_ymd_opt(year,month,day).unwrap(), complete: false, progress: 0}
    }
    pub fn progress_add(&mut self) -> i32 {
        self.progress += 1;
        self.progress
    }
    pub fn progress_set(&mut self,i: i32) -> i32 {
        self.progress = i;
        self.progress
    }
    pub fn progress_get(self) -> i32{
        self.progress
    }
    pub fn complete(&mut self) -> bool{
        self.complete = true;
        self.complete
    }
}