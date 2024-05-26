<h1> Data Serialization (JSON Convention) </h1>
<h2> Front-end </h2>


<h3> JSON format for displaying question </h3>

This JSON format is used when sending data from back-end to front-end.

<b>General structure</b>

```

{

    "question_id": [question_type]_[skill-id]_[question-index],

    "body": {
        "question":[question],
        "label": [for CAT question],
        "choices": [choices],
    }
}


```


Example: 

```
#index.json
{

"question_id": "MCQ_789_1",

"body": {
    "question": "A flashing red traffic light signifies that a driver should do what?",
    "label": null,
    "choices": [
        "stop", "shutup"
    ]
}

}


```

<h3> Sending response from front-end to back-end </h3>

<h3>Input</h3> 

This JSON format is used when sending data from front-end to back-end to check the answer. 


```

{

    "question_id": String [question_type]_[skill-id]_[question-index],
    "body": {
        "answer": Vec<string>, 
    }
}


```

The child object `body.answer` and `body.answer_index` are included to add an extra-step verification. (It can be removed later on.) 

Example 
```
{
        "question_id": "MCQ_789_0",
        "body": {
            "answer": ["stop"]
        }
}
```
```
{
        "question_id": "CAT_789_0",
        "body": {
            "answer": ["vegetable","fruit"]
        }
}
```
<h3>Response</h3> 

To be designed.


<h2> Back-end </h2>


The question and answer of each question are stored in the same JSON object as shown in the example below.

```
#JSON template for one problem
{
    "problem_id": "MCQ_789_0",
    "body": {
        "question": "A flashing red traffic light signifies that a driver should do what?",
        "label": null,
        "choices": [
            "stop",
            "shutup"
        ],
        "answer":"stop"
    }
}


```

The structure is almost similar to JSON format for sending response, but without `answer` and `answer_index`.

<h2> General format</h2>

<h3> 

`question_type` 

 </h3>

Multiple choice: `MCQ` <br>
True false: `TFQ `       <br> 
Categorization: `CAT`  <br>
Error checking: `ERC`   <br>
Error checking (Syntax): `ECS`   <br>

In Rust, all question types are represented by `enum QuestionType`. 

```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    MCQ, TFQ, CAT, ERC
}
```

Even though `MCQ, TFQ, ERC` have the same input-output format, this differentiation is preferred for error handling. (Check `init_server/src/server/format/question_type.rs`) Please check out the following templates for clarification.

Noted that `image` feature has not been implemented yet.

<h3> 

`body` 

 </h3>

<h3> 

Multiple choice questions  `MCQ` 

</h3>

```
"body": {
    "question": "A flashing red traffic light signifies that a driver should do what?",
    "label": null,
    "choices": [
        "stop", "shutup"
    ]
}

```

<h3> 

True-false questions  `TFQ` 

</h3>

```
"body": {
    "question": "London bridge is falling down.",
    "label": null,
    "choices": [
        "True", "False"
    ]
}

```

Noted that `"choices": ["True", "False", "Not given]` is still acceptable.

<h3> 

Categorization questions*  `CAT` 

</h3>


```
 "body": {
    "question": "Fruits and veggies classification",
    "label": ["Fruits", "Veggies"],
    "choices": [
        "Apple", "Carrot", "Raddish"
    ]
    }

```

<h3> 

Error-checking `ERC/ECS` 

</h3>


```
 "body": {
    "question": "Identify which line should be adjusted to be correct.",
    "label": ["int main():\n print("hello world")"],
    "choices": [
        "All"
    ]
    }

```

```
 "body": {
    "question": "Identify which line should be adjusted to be correct.",
    "label": ["int main():\n print("hello world")"],
    "choices": [
        "1", "3", "5"
    ]
    }

```