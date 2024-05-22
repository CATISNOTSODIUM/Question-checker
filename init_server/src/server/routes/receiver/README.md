<h2> Usage description </h2>

<h3> Convert raw JSON to usable struct </h3>
First, the JSON data sent from frontend will be parsed through this struct.

```
#[derive(Deserialize, Debug)] 
pub struct MyBareResponse { 
    pub question_id: String,
    pub question_type: String,
    pub body: MyBareBody,
}

#[derive(Deserialize, Debug)]
pub struct MyBareBody {
    pub answer: String,
    pub answer_index: u32,
}
```

Before parsing the data into the `MyResponse` struct, `handler::convert_format` performs a **preliminary** check to ensure the data format remains valid. 

```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyResponse { 
    pub question_id: String,
    pub question_type: QuestionType,
    pub body: MyBody,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MyBody {
    pub answer: String,
    pub answer_index: u32,
}

```

<h3> Question types </h3>

Multiple choice: `MCQ` <br>
True false: `TFQ `       <br> 
Categorization: `CAT`  <br>
Error checking: `ERC`   <br>

```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MCQ, TFQ, CAT, ERC
}
```