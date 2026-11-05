use std::{fmt::format, str::FromStr};



#[derive(Debug)]
pub enum TaskStatusCheck{
    TODO,
    DONE
}

#[derive(Debug)]
pub enum TaskImportance{
    LOW,
    MEDIUM,
    HIGH,
    NONE
}

impl FromStr for TaskStatusCheck{
    type Err = String; //error type returns when text doesnt match no variants

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str(){
            "todo" => Ok(Self::TODO),
            "done" => Ok(Self::DONE),
            other => Err(format!("unknown status: {other}")),

        }
    }


}

impl FromStr for TaskImportance{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str(){
            "low" => Ok(Self::LOW),
            "medium" => Ok(Self::MEDIUM),
            "high" => Ok(Self::HIGH),
            "none" => Ok(Self::NONE),
            other => Err(format!("unknown importance: {other}"))
        }
    }
}
