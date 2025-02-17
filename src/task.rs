/*A task is somthing that the user wants to keep track.
Example: Brushing teeth*/
use chrono::prelude::*;
use crate::dot::Dot;
enum CompleteFrequency{
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Yearly,
}
pub struct Task{
    //Name of task
    name: String,
    //How often will the task be presneted to the user
    frequency: CompleteFrequency,
    //List of dots assocated with 
    list: Vec<Dot>,
    //Date when task tracking began needed to calulate Stats
    start_date: NaiveDate,
    //Goal of times task is needed to done by complete frequency
    goal: i32,
}
impl Task{
    pub fn new(name_i: String,frequency_i: CompleteFrequency,list_size: i32,year: i32, month: u32, day: u32,goal_i: i32,) -> Task{
        Task {name: name_i, frequency: frequency_i, list: Vec::with_capacity(list_size.try_into().unwrap()),
            start_date: NaiveDate::from_ymd_opt(year,month,day).unwrap(), goal: goal_i}
    }
}