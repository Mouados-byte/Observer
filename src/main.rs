use std::default::Default;
use std::fmt;

#[derive(Clone, Debug)]
struct DataTemplate {
    data: Vec<Data>,
}

impl DataTemplate {
    fn create_data(
        &mut self,
        name: String,
        content: Option<DataTypes>,
        subscribers: Option<Vec<u32>>,
    ) -> u32 {
        let id = self.data.len().try_into().unwrap_or(0) + 1;
        let data = Data::new(
            id,
            name,
            content,
            subscribers,
        );
        self.data.push(data);
        return id;
    }

    fn add_data(&mut self, data: Data) -> Data {
        let data_clone = data.clone();
        self.data.push(data);
        return data_clone;
    }

    fn remove_data(&mut self, id: u32) {
        let index = self
            .data
            .iter()
            .position(|data| data.id == id)
            .expect("No Data with that id exist");
        self.data.remove(index);
    }

    fn find(&self, id: u32) -> Data{
        for i in &self.data{
            if i.id == id{
                return i.clone();
            }
        }
        println!("{id}");
        panic!("Id not existing")
    }

    fn update(&mut self, id: u32, changed_content: Option<DataTypes>) {
        if let Some(data_index) = self.data.iter().position(|data| data.id == id) {
            let content = changed_content.expect("Value should exist");
    
            let (content_type, data_content): (ContentType, String) = content.parse_data_type();
            self.data[data_index].content_type = content_type;
            self.data[data_index].content = data_content;

            let subscribers = self.data[data_index].subscribers.clone();
            
            println!("{:?}", subscribers);
            for subscriber in subscribers{
                if let Some(subscriber_index) = self.data.iter().position(|data| data.id == subscriber){
                    println!("Change happens to {}", subscriber);
                    println!("affected content {}", self.data[data_index].content);
                    self.data[subscriber_index].content = self.data[data_index].clone().content;
                    println!("{:?}", self.data[subscriber_index])
                }
            }
            println!("{:?}", self.data)
        }
    }
}

#[derive(Clone, Debug)]
struct Data {
    id: u32,
    name: String,
    content_type: ContentType,
    content: String,
    subscribers: Vec<u32>
}

impl Data {
    fn new(
        id: u32,
        name: String,
        content: Option<DataTypes>,
        subscribers: Option<Vec<u32>>,
    ) -> Self {
        let mut content_type: ContentType;
        let mut data_content: String;
        match content {
            Some(c) => {
                (content_type, data_content) = c.parse_data_type()
            },
            _ => {
                (content_type, data_content) = (ContentType::StringType, String::from("Default Value"))
            }
        }
        Data {
            id,
            name,
            content_type,
            content: data_content,
            subscribers: subscribers.unwrap_or(Vec::new())
        }
    }

    fn parse_content(&self) -> DataTypes {
        match self.content_type {
            ContentType::StringType => {
                return DataTypes::StringType(self.content.clone());
            }
            ContentType::IntegerType => {
                return DataTypes::IntegerType(
                    self.content
                        .parse::<i32>()
                        .expect("Failed to parse integer"),
                );
            }
            ContentType::FloatType => {
                return DataTypes::FloatType(
                    self.content.parse::<f32>().expect("Failed to parse float"),
                );
            }
            ContentType::BooleanType => {
                return DataTypes::BooleanType(
                    (self.content.to_lowercase() == ("true".to_string()))
                        | (self.content == ("1".to_string())),
                );
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ContentType {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
}


enum DataTypes {
    StringType(String),
    IntegerType(i32),
    FloatType(f32),
    BooleanType(bool),
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTypes::StringType(content) => {
                write!(f, "{}", content)
            }
            DataTypes::IntegerType(content) => {
                write!(f, "{}", content)
            }
            DataTypes::FloatType(content) => {
                write!(f, "{}", content)
            }
            DataTypes::BooleanType(content) => {
                write!(f, "{}", content)
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

impl DataTypes {
    fn parse_data_type(&self) -> (ContentType, String) {
        let content_type: ContentType;
        let data_content: String;
        match self {
            DataTypes::StringType(t) => {
                content_type = ContentType::StringType;
                data_content = t.clone();
            }
            DataTypes::IntegerType(t) => {
                content_type = ContentType::StringType;
                data_content = t.to_string();
            }
            DataTypes::FloatType(t) => {
                content_type = ContentType::StringType;
                data_content = t.to_string();
            }
            DataTypes::BooleanType(t) => {
                content_type = ContentType::StringType;
                data_content = if *t {
                    String::from("True")
                } else {
                    String::from("False")
                };
            }
            _ => {
                panic!("Type doesnt exist")
            }
        }
        return (content_type, data_content);
    }
}

fn main() {
    let mut data_manager = DataTemplate { data: Vec::new() };
    let data_id1 = data_manager.create_data(
        String::from("Data1"),
        Some(DataTypes::StringType(String::from("Hello"))),
        None,
    );

    let data_id2 = data_manager.create_data(
        String::from("Data2"),
        Some(DataTypes::IntegerType(42)),
        Some(vec![data_id1]),
    );

    let data_id3 = data_manager.create_data(
        String::from("Data3"),
        Some(DataTypes::FloatType(3.14159)),
        Some(vec![data_id2]),
    );

    let data_id4 = data_manager.create_data(
        String::from("Data4"),
        Some(DataTypes::BooleanType(true)),
        Some(vec![data_id3]),
    );

    for i in &data_manager.data {
        println!("{}", i.parse_content());
    }

    data_manager.update(data_id2, Some(DataTypes::StringType(String::from("Test"))));
    data_manager.update(data_id4, Some(DataTypes::BooleanType(false)));
    println!("{:?}", data_manager.data);
}
